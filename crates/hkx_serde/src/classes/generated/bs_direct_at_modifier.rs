//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSDirectAtModifier`
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

/// `BSDirectAtModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 176
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x19a005c0`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BsDirectAtModifier<'a> {
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
    /// -   name:`"directAtTarget"`
    /// -   type: `hkBool`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub direct_at_target: bool,
    /// # C++ Class Fields Info
    /// -   name:`"sourceBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 46
    /// -  flags: `FLAGS_NONE`
    pub source_bone_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"startBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub start_bone_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"endBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 50
    /// -  flags: `FLAGS_NONE`
    pub end_bone_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"limitHeadingDegrees"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub limit_heading_degrees: f32,
    /// # C++ Class Fields Info
    /// -   name:`"limitPitchDegrees"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub limit_pitch_degrees: f32,
    /// # C++ Class Fields Info
    /// -   name:`"offsetHeadingDegrees"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub offset_heading_degrees: f32,
    /// # C++ Class Fields Info
    /// -   name:`"offsetPitchDegrees"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub offset_pitch_degrees: f32,
    /// # C++ Class Fields Info
    /// -   name:`"onGain"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub on_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"offGain"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    pub off_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"targetLocation"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub target_location: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"userInfo"`
    /// -   type: `hkUint32`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub user_info: u32,
    /// # C++ Class Fields Info
    /// -   name:`"directAtCamera"`
    /// -   type: `hkBool`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    pub direct_at_camera: bool,
    /// # C++ Class Fields Info
    /// -   name:`"directAtCameraX"`
    /// -   type: `hkReal`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    pub direct_at_camera_x: f32,
    /// # C++ Class Fields Info
    /// -   name:`"directAtCameraY"`
    /// -   type: `hkReal`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    pub direct_at_camera_y: f32,
    /// # C++ Class Fields Info
    /// -   name:`"directAtCameraZ"`
    /// -   type: `hkReal`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    pub direct_at_camera_z: f32,
    /// # C++ Class Fields Info
    /// -   name:`"active"`
    /// -   type: `hkBool`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    pub active: bool,
    /// # C++ Class Fields Info
    /// -   name:`"currentHeadingOffset"`
    /// -   type: `hkReal`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    pub current_heading_offset: f32,
    /// # C++ Class Fields Info
    /// -   name:`"currentPitchOffset"`
    /// -   type: `hkReal`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    pub current_pitch_offset: f32,
    /// # C++ Class Fields Info
    /// -   name:`"timeStep"`
    /// -   type: `hkReal`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub time_step: f32,
    /// # C++ Class Fields Info
    /// -   name:`"pSkeletonMemory"`
    /// -   type: `void*`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub p_skeleton_memory: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"hasTarget"`
    /// -   type: `hkBool`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub has_target: bool,
    /// # C++ Class Fields Info
    /// -   name:`"directAtTargetLocation"`
    /// -   type: `hkVector4`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub direct_at_target_location: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"boneChainIndices"`
    /// -   type: `hkArray<void>`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub bone_chain_indices: HkArrayRef<()>,
}

impl Serialize for BsDirectAtModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<BsDirectAtModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BsDirectAtModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<BsDirectAtModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<BsDirectAtModifierVisitor<'a>>> for BsDirectAtModifier<'a> {
    fn from(_values: Vec<BsDirectAtModifierVisitor<'a>>) -> Self {
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
            let mut direct_at_target = None;
            let mut source_bone_index = None;
            let mut start_bone_index = None;
            let mut end_bone_index = None;
            let mut limit_heading_degrees = None;
            let mut limit_pitch_degrees = None;
            let mut offset_heading_degrees = None;
            let mut offset_pitch_degrees = None;
            let mut on_gain = None;
            let mut off_gain = None;
            let mut target_location = None;
            let mut user_info = None;
            let mut direct_at_camera = None;
            let mut direct_at_camera_x = None;
            let mut direct_at_camera_y = None;
            let mut direct_at_camera_z = None;
            let mut active = None;
            let mut current_heading_offset = None;
            let mut current_pitch_offset = None;
            let mut time_step = None;
            let mut p_skeleton_memory = None;
            let mut has_target = None;
            let mut direct_at_target_location = None;
            let mut bone_chain_indices = None;


        for _value in _values {
            match _value {
                BsDirectAtModifierVisitor::Enable(m) => enable = Some(m),
                BsDirectAtModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                BsDirectAtModifierVisitor::UserData(m) => user_data = Some(m),
                BsDirectAtModifierVisitor::Name(m) => name = Some(m),
                BsDirectAtModifierVisitor::Id(m) => id = Some(m),
                BsDirectAtModifierVisitor::CloneState(m) => clone_state = Some(m),
                BsDirectAtModifierVisitor::PadNode(m) => pad_node = Some(m),
                BsDirectAtModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                BsDirectAtModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                BsDirectAtModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                BsDirectAtModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                BsDirectAtModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                BsDirectAtModifierVisitor::DirectAtTarget(m) => direct_at_target = Some(m),
                BsDirectAtModifierVisitor::SourceBoneIndex(m) => source_bone_index = Some(m),
                BsDirectAtModifierVisitor::StartBoneIndex(m) => start_bone_index = Some(m),
                BsDirectAtModifierVisitor::EndBoneIndex(m) => end_bone_index = Some(m),
                BsDirectAtModifierVisitor::LimitHeadingDegrees(m) => limit_heading_degrees = Some(m),
                BsDirectAtModifierVisitor::LimitPitchDegrees(m) => limit_pitch_degrees = Some(m),
                BsDirectAtModifierVisitor::OffsetHeadingDegrees(m) => offset_heading_degrees = Some(m),
                BsDirectAtModifierVisitor::OffsetPitchDegrees(m) => offset_pitch_degrees = Some(m),
                BsDirectAtModifierVisitor::OnGain(m) => on_gain = Some(m),
                BsDirectAtModifierVisitor::OffGain(m) => off_gain = Some(m),
                BsDirectAtModifierVisitor::TargetLocation(m) => target_location = Some(m),
                BsDirectAtModifierVisitor::UserInfo(m) => user_info = Some(m),
                BsDirectAtModifierVisitor::DirectAtCamera(m) => direct_at_camera = Some(m),
                BsDirectAtModifierVisitor::DirectAtCameraX(m) => direct_at_camera_x = Some(m),
                BsDirectAtModifierVisitor::DirectAtCameraY(m) => direct_at_camera_y = Some(m),
                BsDirectAtModifierVisitor::DirectAtCameraZ(m) => direct_at_camera_z = Some(m),
                BsDirectAtModifierVisitor::Active(m) => active = Some(m),
                BsDirectAtModifierVisitor::CurrentHeadingOffset(m) => current_heading_offset = Some(m),
                BsDirectAtModifierVisitor::CurrentPitchOffset(m) => current_pitch_offset = Some(m),
                BsDirectAtModifierVisitor::TimeStep(m) => time_step = Some(m),
                BsDirectAtModifierVisitor::PSkeletonMemory(m) => p_skeleton_memory = Some(m),
                BsDirectAtModifierVisitor::HasTarget(m) => has_target = Some(m),
                BsDirectAtModifierVisitor::DirectAtTargetLocation(m) => direct_at_target_location = Some(m),
                BsDirectAtModifierVisitor::BoneChainIndices(m) => bone_chain_indices = Some(m),

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
            direct_at_target: direct_at_target.unwrap_or_default().into_inner(),
            source_bone_index: source_bone_index.unwrap_or_default().into_inner(),
            start_bone_index: start_bone_index.unwrap_or_default().into_inner(),
            end_bone_index: end_bone_index.unwrap_or_default().into_inner(),
            limit_heading_degrees: limit_heading_degrees.unwrap_or_default().into_inner(),
            limit_pitch_degrees: limit_pitch_degrees.unwrap_or_default().into_inner(),
            offset_heading_degrees: offset_heading_degrees.unwrap_or_default().into_inner(),
            offset_pitch_degrees: offset_pitch_degrees.unwrap_or_default().into_inner(),
            on_gain: on_gain.unwrap_or_default().into_inner(),
            off_gain: off_gain.unwrap_or_default().into_inner(),
            target_location: target_location.unwrap_or_default().into_inner(),
            user_info: user_info.unwrap_or_default().into_inner(),
            direct_at_camera: direct_at_camera.unwrap_or_default().into_inner(),
            direct_at_camera_x: direct_at_camera_x.unwrap_or_default().into_inner(),
            direct_at_camera_y: direct_at_camera_y.unwrap_or_default().into_inner(),
            direct_at_camera_z: direct_at_camera_z.unwrap_or_default().into_inner(),
            active: active.unwrap_or_default().into_inner(),
            current_heading_offset: current_heading_offset.unwrap_or_default().into_inner(),
            current_pitch_offset: current_pitch_offset.unwrap_or_default().into_inner(),
            time_step: time_step.unwrap_or_default().into_inner(),
            p_skeleton_memory: p_skeleton_memory.unwrap_or_default().into_inner(),
            has_target: has_target.unwrap_or_default().into_inner(),
            direct_at_target_location: direct_at_target_location.unwrap_or_default().into_inner(),
            bone_chain_indices: bone_chain_indices.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&BsDirectAtModifier<'a>> for Vec<BsDirectAtModifierVisitor<'a>> {
    fn from(data: &BsDirectAtModifier<'a>) -> Self {
        vec![
            BsDirectAtModifierVisitor::Enable(data.enable.into()),
            BsDirectAtModifierVisitor::PadModifier(data.pad_modifier.clone()),
            BsDirectAtModifierVisitor::UserData(data.user_data.into()),
            BsDirectAtModifierVisitor::Name(data.name.clone().into()),
            BsDirectAtModifierVisitor::Id(data.id.into()),
            BsDirectAtModifierVisitor::CloneState(data.clone_state.into()),
            BsDirectAtModifierVisitor::PadNode(data.pad_node.clone()),
            BsDirectAtModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            BsDirectAtModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            BsDirectAtModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            BsDirectAtModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            BsDirectAtModifierVisitor::ReferenceCount(data.reference_count.into()),
            BsDirectAtModifierVisitor::DirectAtTarget(data.direct_at_target.into()),
            BsDirectAtModifierVisitor::SourceBoneIndex(data.source_bone_index.into()),
            BsDirectAtModifierVisitor::StartBoneIndex(data.start_bone_index.into()),
            BsDirectAtModifierVisitor::EndBoneIndex(data.end_bone_index.into()),
            BsDirectAtModifierVisitor::LimitHeadingDegrees(data.limit_heading_degrees.into()),
            BsDirectAtModifierVisitor::LimitPitchDegrees(data.limit_pitch_degrees.into()),
            BsDirectAtModifierVisitor::OffsetHeadingDegrees(data.offset_heading_degrees.into()),
            BsDirectAtModifierVisitor::OffsetPitchDegrees(data.offset_pitch_degrees.into()),
            BsDirectAtModifierVisitor::OnGain(data.on_gain.into()),
            BsDirectAtModifierVisitor::OffGain(data.off_gain.into()),
            BsDirectAtModifierVisitor::TargetLocation(data.target_location.into()),
            BsDirectAtModifierVisitor::UserInfo(data.user_info.into()),
            BsDirectAtModifierVisitor::DirectAtCamera(data.direct_at_camera.into()),
            BsDirectAtModifierVisitor::DirectAtCameraX(data.direct_at_camera_x.into()),
            BsDirectAtModifierVisitor::DirectAtCameraY(data.direct_at_camera_y.into()),
            BsDirectAtModifierVisitor::DirectAtCameraZ(data.direct_at_camera_z.into()),
            BsDirectAtModifierVisitor::Active(data.active.into()),
            BsDirectAtModifierVisitor::CurrentHeadingOffset(data.current_heading_offset.into()),
            BsDirectAtModifierVisitor::CurrentPitchOffset(data.current_pitch_offset.into()),
            BsDirectAtModifierVisitor::TimeStep(data.time_step.into()),
            BsDirectAtModifierVisitor::PSkeletonMemory(data.p_skeleton_memory.clone().into()),
            BsDirectAtModifierVisitor::HasTarget(data.has_target.into()),
            BsDirectAtModifierVisitor::DirectAtTargetLocation(data.direct_at_target_location.into()),
            BsDirectAtModifierVisitor::BoneChainIndices(data.bone_chain_indices.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for BsDirectAtModifier<'de> {
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
enum BsDirectAtModifierVisitor<'a> {
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
    #[serde(rename = "directAtTarget")]
    DirectAtTarget(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "sourceBoneIndex")]
    SourceBoneIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "startBoneIndex")]
    StartBoneIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "endBoneIndex")]
    EndBoneIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "limitHeadingDegrees")]
    LimitHeadingDegrees(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "limitPitchDegrees")]
    LimitPitchDegrees(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "offsetHeadingDegrees")]
    OffsetHeadingDegrees(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "offsetPitchDegrees")]
    OffsetPitchDegrees(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "onGain")]
    OnGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "offGain")]
    OffGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "targetLocation")]
    TargetLocation(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "userInfo")]
    UserInfo(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "directAtCamera")]
    DirectAtCamera(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "directAtCameraX")]
    DirectAtCameraX(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "directAtCameraY")]
    DirectAtCameraY(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "directAtCameraZ")]
    DirectAtCameraZ(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "active")]
    Active(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "currentHeadingOffset")]
    CurrentHeadingOffset(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "currentPitchOffset")]
    CurrentPitchOffset(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "timeStep", skip_serializing)]
    TimeStep(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "pSkeletonMemory", skip_serializing)]
    PSkeletonMemory(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "hasTarget", skip_serializing)]
    HasTarget(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "directAtTargetLocation", skip_serializing)]
    DirectAtTargetLocation(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "boneChainIndices", skip_serializing)]
    BoneChainIndices(HkArrayRef<()>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsDirectAtModifierVisitor<'de>, "@name",
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
    ("directAtTarget" => DirectAtTarget(Primitive<bool>)),
    ("sourceBoneIndex" => SourceBoneIndex(Primitive<i16>)),
    ("startBoneIndex" => StartBoneIndex(Primitive<i16>)),
    ("endBoneIndex" => EndBoneIndex(Primitive<i16>)),
    ("limitHeadingDegrees" => LimitHeadingDegrees(Primitive<f32>)),
    ("limitPitchDegrees" => LimitPitchDegrees(Primitive<f32>)),
    ("offsetHeadingDegrees" => OffsetHeadingDegrees(Primitive<f32>)),
    ("offsetPitchDegrees" => OffsetPitchDegrees(Primitive<f32>)),
    ("onGain" => OnGain(Primitive<f32>)),
    ("offGain" => OffGain(Primitive<f32>)),
    ("targetLocation" => TargetLocation(Primitive<Vector4<f32>>)),
    ("userInfo" => UserInfo(Primitive<u32>)),
    ("directAtCamera" => DirectAtCamera(Primitive<bool>)),
    ("directAtCameraX" => DirectAtCameraX(Primitive<f32>)),
    ("directAtCameraY" => DirectAtCameraY(Primitive<f32>)),
    ("directAtCameraZ" => DirectAtCameraZ(Primitive<f32>)),
    ("active" => Active(Primitive<bool>)),
    ("currentHeadingOffset" => CurrentHeadingOffset(Primitive<f32>)),
    ("currentPitchOffset" => CurrentPitchOffset(Primitive<f32>)),
    ("timeStep" => TimeStep(Primitive<f32>)),
    ("pSkeletonMemory" => PSkeletonMemory(Primitive<Cow<'de, str>>)),
    ("hasTarget" => HasTarget(Primitive<bool>)),
    ("directAtTargetLocation" => DirectAtTargetLocation(Primitive<Vector4<f32>>)),
    ("boneChainIndices" => BoneChainIndices(HkArrayRef<()>)),
}
