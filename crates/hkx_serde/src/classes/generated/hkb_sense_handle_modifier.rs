//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbSenseHandleModifier`
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

/// `hkbSenseHandleModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 160
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x2a064d99`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbSenseHandleModifier<'a> {
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
    /// -   name:`"handle"`
    /// -   type: `struct hkbHandle`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub handle: SingleClass<HkbHandle<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"sensorLocalOffset"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub sensor_local_offset: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"ranges"`
    /// -   type: `hkArray<struct hkbSenseHandleModifierRange>`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub ranges: HkArrayClass<HkbSenseHandleModifierRange<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"handleOut"`
    /// -   type: `struct hkbHandle*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    pub handle_out: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"handleIn"`
    /// -   type: `struct hkbHandle*`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    pub handle_in: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"localFrameName"`
    /// -   type: `hkStringPtr`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    pub local_frame_name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"sensorLocalFrameName"`
    /// -   type: `hkStringPtr`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    pub sensor_local_frame_name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"minDistance"`
    /// -   type: `hkReal`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    pub min_distance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxDistance"`
    /// -   type: `hkReal`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    pub max_distance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"distanceOut"`
    /// -   type: `hkReal`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE`
    pub distance_out: f32,
    /// # C++ Class Fields Info
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE`
    pub collision_filter_info: u32,
    /// # C++ Class Fields Info
    /// -   name:`"sensorRagdollBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    pub sensor_ragdoll_bone_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"sensorAnimationBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 142
    /// -  flags: `FLAGS_NONE`
    pub sensor_animation_bone_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"sensingMode"`
    /// -   type: `enum SensingMode`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    pub sensing_mode: SensingMode,
    /// # C++ Class Fields Info
    /// -   name:`"extrapolateSensorPosition"`
    /// -   type: `hkBool`
    /// - offset: 145
    /// -  flags: `FLAGS_NONE`
    pub extrapolate_sensor_position: bool,
    /// # C++ Class Fields Info
    /// -   name:`"keepFirstSensedHandle"`
    /// -   type: `hkBool`
    /// - offset: 146
    /// -  flags: `FLAGS_NONE`
    pub keep_first_sensed_handle: bool,
    /// # C++ Class Fields Info
    /// -   name:`"foundHandleOut"`
    /// -   type: `hkBool`
    /// - offset: 147
    /// -  flags: `FLAGS_NONE`
    pub found_handle_out: bool,
    /// # C++ Class Fields Info
    /// -   name:`"timeSinceLastModify"`
    /// -   type: `hkReal`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub time_since_last_modify: f32,
    /// # C++ Class Fields Info
    /// -   name:`"rangeIndexForEventToSendNextUpdate"`
    /// -   type: `hkInt32`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub range_index_for_event_to_send_next_update: i32,
}

impl Serialize for HkbSenseHandleModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbSenseHandleModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbSenseHandleModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbSenseHandleModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbSenseHandleModifierVisitor<'a>>> for HkbSenseHandleModifier<'a> {
    fn from(_values: Vec<HkbSenseHandleModifierVisitor<'a>>) -> Self {
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
            let mut handle = None;
            let mut sensor_local_offset = None;
            let mut ranges = None;
            let mut handle_out = None;
            let mut handle_in = None;
            let mut local_frame_name = None;
            let mut sensor_local_frame_name = None;
            let mut min_distance = None;
            let mut max_distance = None;
            let mut distance_out = None;
            let mut collision_filter_info = None;
            let mut sensor_ragdoll_bone_index = None;
            let mut sensor_animation_bone_index = None;
            let mut sensing_mode = None;
            let mut extrapolate_sensor_position = None;
            let mut keep_first_sensed_handle = None;
            let mut found_handle_out = None;
            let mut time_since_last_modify = None;
            let mut range_index_for_event_to_send_next_update = None;


        for _value in _values {
            match _value {
                HkbSenseHandleModifierVisitor::Enable(m) => enable = Some(m),
                HkbSenseHandleModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                HkbSenseHandleModifierVisitor::UserData(m) => user_data = Some(m),
                HkbSenseHandleModifierVisitor::Name(m) => name = Some(m),
                HkbSenseHandleModifierVisitor::Id(m) => id = Some(m),
                HkbSenseHandleModifierVisitor::CloneState(m) => clone_state = Some(m),
                HkbSenseHandleModifierVisitor::PadNode(m) => pad_node = Some(m),
                HkbSenseHandleModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbSenseHandleModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbSenseHandleModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbSenseHandleModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbSenseHandleModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbSenseHandleModifierVisitor::Handle(m) => handle = Some(m),
                HkbSenseHandleModifierVisitor::SensorLocalOffset(m) => sensor_local_offset = Some(m),
                HkbSenseHandleModifierVisitor::Ranges(m) => ranges = Some(m),
                HkbSenseHandleModifierVisitor::HandleOut(m) => handle_out = Some(m),
                HkbSenseHandleModifierVisitor::HandleIn(m) => handle_in = Some(m),
                HkbSenseHandleModifierVisitor::LocalFrameName(m) => local_frame_name = Some(m),
                HkbSenseHandleModifierVisitor::SensorLocalFrameName(m) => sensor_local_frame_name = Some(m),
                HkbSenseHandleModifierVisitor::MinDistance(m) => min_distance = Some(m),
                HkbSenseHandleModifierVisitor::MaxDistance(m) => max_distance = Some(m),
                HkbSenseHandleModifierVisitor::DistanceOut(m) => distance_out = Some(m),
                HkbSenseHandleModifierVisitor::CollisionFilterInfo(m) => collision_filter_info = Some(m),
                HkbSenseHandleModifierVisitor::SensorRagdollBoneIndex(m) => sensor_ragdoll_bone_index = Some(m),
                HkbSenseHandleModifierVisitor::SensorAnimationBoneIndex(m) => sensor_animation_bone_index = Some(m),
                HkbSenseHandleModifierVisitor::SensingMode(m) => sensing_mode = Some(m),
                HkbSenseHandleModifierVisitor::ExtrapolateSensorPosition(m) => extrapolate_sensor_position = Some(m),
                HkbSenseHandleModifierVisitor::KeepFirstSensedHandle(m) => keep_first_sensed_handle = Some(m),
                HkbSenseHandleModifierVisitor::FoundHandleOut(m) => found_handle_out = Some(m),
                HkbSenseHandleModifierVisitor::TimeSinceLastModify(m) => time_since_last_modify = Some(m),
                HkbSenseHandleModifierVisitor::RangeIndexForEventToSendNextUpdate(m) => range_index_for_event_to_send_next_update = Some(m),

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
            handle: handle.unwrap_or_default(),
            sensor_local_offset: sensor_local_offset.unwrap_or_default().into_inner(),
            ranges: ranges.unwrap_or_default(),
            handle_out: handle_out.unwrap_or_default().into_inner(),
            handle_in: handle_in.unwrap_or_default().into_inner(),
            local_frame_name: local_frame_name.unwrap_or_default().into_inner(),
            sensor_local_frame_name: sensor_local_frame_name.unwrap_or_default().into_inner(),
            min_distance: min_distance.unwrap_or_default().into_inner(),
            max_distance: max_distance.unwrap_or_default().into_inner(),
            distance_out: distance_out.unwrap_or_default().into_inner(),
            collision_filter_info: collision_filter_info.unwrap_or_default().into_inner(),
            sensor_ragdoll_bone_index: sensor_ragdoll_bone_index.unwrap_or_default().into_inner(),
            sensor_animation_bone_index: sensor_animation_bone_index.unwrap_or_default().into_inner(),
            sensing_mode: sensing_mode.unwrap_or_default().into_inner(),
            extrapolate_sensor_position: extrapolate_sensor_position.unwrap_or_default().into_inner(),
            keep_first_sensed_handle: keep_first_sensed_handle.unwrap_or_default().into_inner(),
            found_handle_out: found_handle_out.unwrap_or_default().into_inner(),
            time_since_last_modify: time_since_last_modify.unwrap_or_default().into_inner(),
            range_index_for_event_to_send_next_update: range_index_for_event_to_send_next_update.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbSenseHandleModifier<'a>> for Vec<HkbSenseHandleModifierVisitor<'a>> {
    fn from(data: &HkbSenseHandleModifier<'a>) -> Self {
        vec![
            HkbSenseHandleModifierVisitor::Enable(data.enable.into()),
            HkbSenseHandleModifierVisitor::PadModifier(data.pad_modifier.clone()),
            HkbSenseHandleModifierVisitor::UserData(data.user_data.into()),
            HkbSenseHandleModifierVisitor::Name(data.name.clone().into()),
            HkbSenseHandleModifierVisitor::Id(data.id.into()),
            HkbSenseHandleModifierVisitor::CloneState(data.clone_state.into()),
            HkbSenseHandleModifierVisitor::PadNode(data.pad_node.clone()),
            HkbSenseHandleModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbSenseHandleModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbSenseHandleModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbSenseHandleModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbSenseHandleModifierVisitor::ReferenceCount(data.reference_count.into()),
            HkbSenseHandleModifierVisitor::Handle(data.handle.clone()),
            HkbSenseHandleModifierVisitor::SensorLocalOffset(data.sensor_local_offset.into()),
            HkbSenseHandleModifierVisitor::Ranges(data.ranges.clone()),
            HkbSenseHandleModifierVisitor::HandleOut(data.handle_out.clone().into()),
            HkbSenseHandleModifierVisitor::HandleIn(data.handle_in.clone().into()),
            HkbSenseHandleModifierVisitor::LocalFrameName(data.local_frame_name.clone().into()),
            HkbSenseHandleModifierVisitor::SensorLocalFrameName(data.sensor_local_frame_name.clone().into()),
            HkbSenseHandleModifierVisitor::MinDistance(data.min_distance.into()),
            HkbSenseHandleModifierVisitor::MaxDistance(data.max_distance.into()),
            HkbSenseHandleModifierVisitor::DistanceOut(data.distance_out.into()),
            HkbSenseHandleModifierVisitor::CollisionFilterInfo(data.collision_filter_info.into()),
            HkbSenseHandleModifierVisitor::SensorRagdollBoneIndex(data.sensor_ragdoll_bone_index.into()),
            HkbSenseHandleModifierVisitor::SensorAnimationBoneIndex(data.sensor_animation_bone_index.into()),
            HkbSenseHandleModifierVisitor::SensingMode(data.sensing_mode.clone().into()),
            HkbSenseHandleModifierVisitor::ExtrapolateSensorPosition(data.extrapolate_sensor_position.into()),
            HkbSenseHandleModifierVisitor::KeepFirstSensedHandle(data.keep_first_sensed_handle.into()),
            HkbSenseHandleModifierVisitor::FoundHandleOut(data.found_handle_out.into()),
            HkbSenseHandleModifierVisitor::TimeSinceLastModify(data.time_since_last_modify.into()),
            HkbSenseHandleModifierVisitor::RangeIndexForEventToSendNextUpdate(data.range_index_for_event_to_send_next_update.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbSenseHandleModifier<'de> {
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
enum HkbSenseHandleModifierVisitor<'a> {
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
    #[serde(rename = "handle", skip_serializing)]
    Handle(SingleClass<HkbHandle<'a>>),
    /// Visitor fields
    #[serde(rename = "sensorLocalOffset")]
    SensorLocalOffset(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "ranges")]
    Ranges(HkArrayClass<HkbSenseHandleModifierRange<'a>>),
    /// Visitor fields
    #[serde(rename = "handleOut")]
    HandleOut(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "handleIn")]
    HandleIn(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "localFrameName")]
    LocalFrameName(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "sensorLocalFrameName")]
    SensorLocalFrameName(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "minDistance")]
    MinDistance(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxDistance")]
    MaxDistance(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "distanceOut")]
    DistanceOut(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "sensorRagdollBoneIndex")]
    SensorRagdollBoneIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "sensorAnimationBoneIndex")]
    SensorAnimationBoneIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "sensingMode")]
    SensingMode(Primitive<SensingMode>),
    /// Visitor fields
    #[serde(rename = "extrapolateSensorPosition")]
    ExtrapolateSensorPosition(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "keepFirstSensedHandle")]
    KeepFirstSensedHandle(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "foundHandleOut")]
    FoundHandleOut(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "timeSinceLastModify", skip_serializing)]
    TimeSinceLastModify(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "rangeIndexForEventToSendNextUpdate", skip_serializing)]
    RangeIndexForEventToSendNextUpdate(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbSenseHandleModifierVisitor<'de>, "@name",
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
    ("handle" => Handle(SingleClass<HkbHandle<'de>>)),
    ("sensorLocalOffset" => SensorLocalOffset(Primitive<Vector4<f32>>)),
    ("ranges" => Ranges(HkArrayClass<HkbSenseHandleModifierRange<'de>>)),
    ("handleOut" => HandleOut(Primitive<Cow<'de, str>>)),
    ("handleIn" => HandleIn(Primitive<Cow<'de, str>>)),
    ("localFrameName" => LocalFrameName(Primitive<Cow<'de, str>>)),
    ("sensorLocalFrameName" => SensorLocalFrameName(Primitive<Cow<'de, str>>)),
    ("minDistance" => MinDistance(Primitive<f32>)),
    ("maxDistance" => MaxDistance(Primitive<f32>)),
    ("distanceOut" => DistanceOut(Primitive<f32>)),
    ("collisionFilterInfo" => CollisionFilterInfo(Primitive<u32>)),
    ("sensorRagdollBoneIndex" => SensorRagdollBoneIndex(Primitive<i16>)),
    ("sensorAnimationBoneIndex" => SensorAnimationBoneIndex(Primitive<i16>)),
    ("sensingMode" => SensingMode(Primitive<SensingMode>)),
    ("extrapolateSensorPosition" => ExtrapolateSensorPosition(Primitive<bool>)),
    ("keepFirstSensedHandle" => KeepFirstSensedHandle(Primitive<bool>)),
    ("foundHandleOut" => FoundHandleOut(Primitive<bool>)),
    ("timeSinceLastModify" => TimeSinceLastModify(Primitive<f32>)),
    ("rangeIndexForEventToSendNextUpdate" => RangeIndexForEventToSendNextUpdate(Primitive<i32>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum SensingMode {
    #[serde(rename = "SENSE_IN_NEARBY_RIGID_BODIES")]
    #[default]
    SenseInNearbyRigidBodies = 0,
    #[serde(rename = "SENSE_IN_RIGID_BODIES_OUTSIDE_THIS_CHARACTER")]
    SenseInRigidBodiesOutsideThisCharacter = 1,
    #[serde(rename = "SENSE_IN_OTHER_CHARACTER_RIGID_BODIES")]
    SenseInOtherCharacterRigidBodies = 2,
    #[serde(rename = "SENSE_IN_THIS_CHARACTER_RIGID_BODIES")]
    SenseInThisCharacterRigidBodies = 3,
    #[serde(rename = "SENSE_IN_GIVEN_CHARACTER_RIGID_BODIES")]
    SenseInGivenCharacterRigidBodies = 4,
    #[serde(rename = "SENSE_IN_GIVEN_RIGID_BODY")]
    SenseInGivenRigidBody = 5,
    #[serde(rename = "SENSE_IN_OTHER_CHARACTER_SKELETON")]
    SenseInOtherCharacterSkeleton = 6,
    #[serde(rename = "SENSE_IN_THIS_CHARACTER_SKELETON")]
    SenseInThisCharacterSkeleton = 7,
    #[serde(rename = "SENSE_IN_GIVEN_CHARACTER_SKELETON")]
    SenseInGivenCharacterSkeleton = 8,
    #[serde(rename = "SENSE_IN_GIVEN_LOCAL_FRAME_GROUP")]
    SenseInGivenLocalFrameGroup = 9,
}
