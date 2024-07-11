//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbGetUpModifier`
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

/// `hkbGetUpModifier`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: true
/// -    parent: `hkbModifier`/`0x96ec5ced`
/// - signature: `0x61cb7ac0`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbGetUpModifier<'a> {
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
    /// -   name:`"groundNormal"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub ground_normal: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub duration: f32,
    /// # C++ Class Fields Info
    /// -   name:`"alignWithGroundDuration"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub align_with_ground_duration: f32,
    /// # C++ Class Fields Info
    /// -   name:`"rootBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    pub root_bone_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"otherBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 74
    /// -  flags: `FLAGS_NONE`
    pub other_bone_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"anotherBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    pub another_bone_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"timeSinceBegin"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub time_since_begin: f32,
    /// # C++ Class Fields Info
    /// -   name:`"timeStep"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub time_step: f32,
    /// # C++ Class Fields Info
    /// -   name:`"initNextModify"`
    /// -   type: `hkBool`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub init_next_modify: bool,
}

impl Serialize for HkbGetUpModifier<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbGetUpModifierVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbGetUpModifier<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbGetUpModifierVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbGetUpModifierVisitor<'a>>> for HkbGetUpModifier<'a> {
    fn from(_values: Vec<HkbGetUpModifierVisitor<'a>>) -> Self {
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
            let mut ground_normal = None;
            let mut duration = None;
            let mut align_with_ground_duration = None;
            let mut root_bone_index = None;
            let mut other_bone_index = None;
            let mut another_bone_index = None;
            let mut time_since_begin = None;
            let mut time_step = None;
            let mut init_next_modify = None;


        for _value in _values {
            match _value {
                HkbGetUpModifierVisitor::Enable(m) => enable = Some(m),
                HkbGetUpModifierVisitor::PadModifier(m) => pad_modifier = Some(m),
                HkbGetUpModifierVisitor::UserData(m) => user_data = Some(m),
                HkbGetUpModifierVisitor::Name(m) => name = Some(m),
                HkbGetUpModifierVisitor::Id(m) => id = Some(m),
                HkbGetUpModifierVisitor::CloneState(m) => clone_state = Some(m),
                HkbGetUpModifierVisitor::PadNode(m) => pad_node = Some(m),
                HkbGetUpModifierVisitor::VariableBindingSet(m) => variable_binding_set = Some(m),
                HkbGetUpModifierVisitor::CachedBindables(m) => cached_bindables = Some(m),
                HkbGetUpModifierVisitor::AreBindablesCached(m) => are_bindables_cached = Some(m),
                HkbGetUpModifierVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbGetUpModifierVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbGetUpModifierVisitor::GroundNormal(m) => ground_normal = Some(m),
                HkbGetUpModifierVisitor::Duration(m) => duration = Some(m),
                HkbGetUpModifierVisitor::AlignWithGroundDuration(m) => align_with_ground_duration = Some(m),
                HkbGetUpModifierVisitor::RootBoneIndex(m) => root_bone_index = Some(m),
                HkbGetUpModifierVisitor::OtherBoneIndex(m) => other_bone_index = Some(m),
                HkbGetUpModifierVisitor::AnotherBoneIndex(m) => another_bone_index = Some(m),
                HkbGetUpModifierVisitor::TimeSinceBegin(m) => time_since_begin = Some(m),
                HkbGetUpModifierVisitor::TimeStep(m) => time_step = Some(m),
                HkbGetUpModifierVisitor::InitNextModify(m) => init_next_modify = Some(m),

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
            ground_normal: ground_normal.unwrap_or_default().into_inner(),
            duration: duration.unwrap_or_default().into_inner(),
            align_with_ground_duration: align_with_ground_duration.unwrap_or_default().into_inner(),
            root_bone_index: root_bone_index.unwrap_or_default().into_inner(),
            other_bone_index: other_bone_index.unwrap_or_default().into_inner(),
            another_bone_index: another_bone_index.unwrap_or_default().into_inner(),
            time_since_begin: time_since_begin.unwrap_or_default().into_inner(),
            time_step: time_step.unwrap_or_default().into_inner(),
            init_next_modify: init_next_modify.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbGetUpModifier<'a>> for Vec<HkbGetUpModifierVisitor<'a>> {
    fn from(data: &HkbGetUpModifier<'a>) -> Self {
        vec![
            HkbGetUpModifierVisitor::Enable(data.enable.into()),
            HkbGetUpModifierVisitor::PadModifier(data.pad_modifier.clone()),
            HkbGetUpModifierVisitor::UserData(data.user_data.into()),
            HkbGetUpModifierVisitor::Name(data.name.clone().into()),
            HkbGetUpModifierVisitor::Id(data.id.into()),
            HkbGetUpModifierVisitor::CloneState(data.clone_state.into()),
            HkbGetUpModifierVisitor::PadNode(data.pad_node.clone()),
            HkbGetUpModifierVisitor::VariableBindingSet(data.variable_binding_set.clone().into()),
            HkbGetUpModifierVisitor::CachedBindables(data.cached_bindables.clone()),
            HkbGetUpModifierVisitor::AreBindablesCached(data.are_bindables_cached.into()),
            HkbGetUpModifierVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbGetUpModifierVisitor::ReferenceCount(data.reference_count.into()),
            HkbGetUpModifierVisitor::GroundNormal(data.ground_normal.into()),
            HkbGetUpModifierVisitor::Duration(data.duration.into()),
            HkbGetUpModifierVisitor::AlignWithGroundDuration(data.align_with_ground_duration.into()),
            HkbGetUpModifierVisitor::RootBoneIndex(data.root_bone_index.into()),
            HkbGetUpModifierVisitor::OtherBoneIndex(data.other_bone_index.into()),
            HkbGetUpModifierVisitor::AnotherBoneIndex(data.another_bone_index.into()),
            HkbGetUpModifierVisitor::TimeSinceBegin(data.time_since_begin.into()),
            HkbGetUpModifierVisitor::TimeStep(data.time_step.into()),
            HkbGetUpModifierVisitor::InitNextModify(data.init_next_modify.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbGetUpModifier<'de> {
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
enum HkbGetUpModifierVisitor<'a> {
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
    #[serde(rename = "groundNormal")]
    GroundNormal(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "duration")]
    Duration(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "alignWithGroundDuration")]
    AlignWithGroundDuration(Primitive<f32>),
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
    #[serde(rename = "timeSinceBegin", skip_serializing)]
    TimeSinceBegin(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "timeStep", skip_serializing)]
    TimeStep(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "initNextModify", skip_serializing)]
    InitNextModify(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbGetUpModifierVisitor<'de>, "@name",
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
    ("groundNormal" => GroundNormal(Primitive<Vector4<f32>>)),
    ("duration" => Duration(Primitive<f32>)),
    ("alignWithGroundDuration" => AlignWithGroundDuration(Primitive<f32>)),
    ("rootBoneIndex" => RootBoneIndex(Primitive<i16>)),
    ("otherBoneIndex" => OtherBoneIndex(Primitive<i16>)),
    ("anotherBoneIndex" => AnotherBoneIndex(Primitive<i16>)),
    ("timeSinceBegin" => TimeSinceBegin(Primitive<f32>)),
    ("timeStep" => TimeStep(Primitive<f32>)),
    ("initNextModify" => InitNextModify(Primitive<bool>)),
}
