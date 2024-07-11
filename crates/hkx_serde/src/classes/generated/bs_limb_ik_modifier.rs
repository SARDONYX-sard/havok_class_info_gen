//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `BSLimbIKModifier`
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

/// `BSLimbIKModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 76
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x8ea971e5`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BsLimbIkModifier<'a> {
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
    /// -   name:`"limitAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub limit_angle_degrees: f32,
    /// # C++ Class Fields Info
    /// -   name:`"currentAngle"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub current_angle: f32,
    /// # C++ Class Fields Info
    /// -   name:`"startBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub start_bone_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"endBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 54
    /// -  flags: `FLAGS_NONE`
    pub end_bone_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"gain"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"boneRadius"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub bone_radius: f32,
    /// # C++ Class Fields Info
    /// -   name:`"castOffset"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub cast_offset: f32,
    /// # C++ Class Fields Info
    /// -   name:`"timeStep"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub time_step: f32,
    /// # C++ Class Fields Info
    /// -   name:`"pSkeletonMemory"`
    /// -   type: `void*`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub p_skeleton_memory: Cow<'a, str>,
}

impl Serialize for BsLimbIkModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<BsLimbIkModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BsLimbIkModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<BsLimbIkModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<BsLimbIkModifierVisitor<'a>>> for BsLimbIkModifier<'a> {
    fn from(_values: Vec<BsLimbIkModifierVisitor<'a>>) -> Self {
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
            let mut limit_angle_degrees = None;
            let mut current_angle = None;
            let mut start_bone_index = None;
            let mut end_bone_index = None;
            let mut gain = None;
            let mut bone_radius = None;
            let mut cast_offset = None;
            let mut time_step = None;
            let mut p_skeleton_memory = None;


        for _value in _values {
            match _value {
                BsLimbIkModifierVisitor::Enable(m) => enable = Some(m),
                BsLimbIkModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                BsLimbIkModifierVisitor::UserData(m) => user_data = Some(m),
                BsLimbIkModifierVisitor::Name(m) => name = Some(m),
                BsLimbIkModifierVisitor::Id(m) => id = Some(m),
                BsLimbIkModifierVisitor::CloneState(m) => clone_state = Some(m),
                BsLimbIkModifierVisitor::PadNode(m) => pad_node = Some(m),
                BsLimbIkModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                BsLimbIkModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                BsLimbIkModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                BsLimbIkModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                BsLimbIkModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                BsLimbIkModifierVisitor::LimitAngleDegrees(m) => limit_angle_degrees = Some(m),
                BsLimbIkModifierVisitor::CurrentAngle(m) => current_angle = Some(m),
                BsLimbIkModifierVisitor::StartBoneIndex(m) => start_bone_index = Some(m),
                BsLimbIkModifierVisitor::EndBoneIndex(m) => end_bone_index = Some(m),
                BsLimbIkModifierVisitor::Gain(m) => gain = Some(m),
                BsLimbIkModifierVisitor::BoneRadius(m) => bone_radius = Some(m),
                BsLimbIkModifierVisitor::CastOffset(m) => cast_offset = Some(m),
                BsLimbIkModifierVisitor::TimeStep(m) => time_step = Some(m),
                BsLimbIkModifierVisitor::PSkeletonMemory(m) => p_skeleton_memory = Some(m),

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
            limit_angle_degrees: limit_angle_degrees.unwrap_or_default().into_inner(),
            current_angle: current_angle.unwrap_or_default().into_inner(),
            start_bone_index: start_bone_index.unwrap_or_default().into_inner(),
            end_bone_index: end_bone_index.unwrap_or_default().into_inner(),
            gain: gain.unwrap_or_default().into_inner(),
            bone_radius: bone_radius.unwrap_or_default().into_inner(),
            cast_offset: cast_offset.unwrap_or_default().into_inner(),
            time_step: time_step.unwrap_or_default().into_inner(),
            p_skeleton_memory: p_skeleton_memory.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&BsLimbIkModifier<'a>> for Vec<BsLimbIkModifierVisitor<'a>> {
    fn from(data: &BsLimbIkModifier<'a>) -> Self {
        vec![
            BsLimbIkModifierVisitor::Enable(data.enable.into()),
            BsLimbIkModifierVisitor::PadModifier(data.pad_modifier.clone()),
            BsLimbIkModifierVisitor::UserData(data.user_data.into()),
            BsLimbIkModifierVisitor::Name(data.name.clone().into()),
            BsLimbIkModifierVisitor::Id(data.id.into()),
            BsLimbIkModifierVisitor::CloneState(data.clone_state.into()),
            BsLimbIkModifierVisitor::PadNode(data.pad_node.clone()),
            BsLimbIkModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            BsLimbIkModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            BsLimbIkModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            BsLimbIkModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            BsLimbIkModifierVisitor::ReferenceCount(data.reference_count.into()),
            BsLimbIkModifierVisitor::LimitAngleDegrees(data.limit_angle_degrees.into()),
            BsLimbIkModifierVisitor::CurrentAngle(data.current_angle.into()),
            BsLimbIkModifierVisitor::StartBoneIndex(data.start_bone_index.into()),
            BsLimbIkModifierVisitor::EndBoneIndex(data.end_bone_index.into()),
            BsLimbIkModifierVisitor::Gain(data.gain.into()),
            BsLimbIkModifierVisitor::BoneRadius(data.bone_radius.into()),
            BsLimbIkModifierVisitor::CastOffset(data.cast_offset.into()),
            BsLimbIkModifierVisitor::TimeStep(data.time_step.into()),
            BsLimbIkModifierVisitor::PSkeletonMemory(data.p_skeleton_memory.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for BsLimbIkModifier<'de> {
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
enum BsLimbIkModifierVisitor<'a> {
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
    #[serde(rename = "limitAngleDegrees")]
    LimitAngleDegrees(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "currentAngle", skip_serializing)]
    CurrentAngle(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "startBoneIndex")]
    StartBoneIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "endBoneIndex")]
    EndBoneIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "gain")]
    Gain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "boneRadius")]
    BoneRadius(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "castOffset")]
    CastOffset(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "timeStep", skip_serializing)]
    TimeStep(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "pSkeletonMemory", skip_serializing)]
    PSkeletonMemory(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    BsLimbIkModifierVisitor<'de>, "@name",
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
    ("limitAngleDegrees" => LimitAngleDegrees(Primitive<f32>)),
    ("currentAngle" => CurrentAngle(Primitive<f32>)),
    ("startBoneIndex" => StartBoneIndex(Primitive<i16>)),
    ("endBoneIndex" => EndBoneIndex(Primitive<i16>)),
    ("gain" => Gain(Primitive<f32>)),
    ("boneRadius" => BoneRadius(Primitive<f32>)),
    ("castOffset" => CastOffset(Primitive<f32>)),
    ("timeStep" => TimeStep(Primitive<f32>)),
    ("pSkeletonMemory" => PSkeletonMemory(Primitive<Cow<'de, str>>)),
}
