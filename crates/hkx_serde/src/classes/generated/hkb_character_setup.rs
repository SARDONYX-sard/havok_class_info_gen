//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbCharacterSetup`
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

/// `hkbCharacterSetup`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xe5a2a413`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbCharacterSetup<'a> {
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
    /// -   name:`"retargetingSkeletonMappers"`
    /// -   type: `hkArray<hkaSkeletonMapper*>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub retargeting_skeleton_mappers: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"animationSkeleton"`
    /// -   type: `struct hkaSkeleton*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub animation_skeleton: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"ragdollToAnimationSkeletonMapper"`
    /// -   type: `struct hkaSkeletonMapper*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub ragdoll_to_animation_skeleton_mapper: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"animationToRagdollSkeletonMapper"`
    /// -   type: `struct hkaSkeletonMapper*`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub animation_to_ragdoll_skeleton_mapper: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"animationBindingSet"`
    /// -   type: `void*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub animation_binding_set: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `struct hkbCharacterData*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub data: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"mirroredSkeleton"`
    /// -   type: `void*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub mirrored_skeleton: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"characterPropertyIdMap"`
    /// -   type: `void*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub character_property_id_map: Cow<'a, str>,
}

impl Serialize for HkbCharacterSetup<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbCharacterSetupVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbCharacterSetup<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbCharacterSetupVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbCharacterSetupVisitor<'a>>> for HkbCharacterSetup<'a> {
    fn from(_values: Vec<HkbCharacterSetupVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut retargeting_skeleton_mappers = None;
            let mut animation_skeleton = None;
            let mut ragdoll_to_animation_skeleton_mapper = None;
            let mut animation_to_ragdoll_skeleton_mapper = None;
            let mut animation_binding_set = None;
            let mut data = None;
            let mut mirrored_skeleton = None;
            let mut character_property_id_map = None;


        for _value in _values {
            match _value {
                HkbCharacterSetupVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbCharacterSetupVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbCharacterSetupVisitor::RetargetingSkeletonMappers(m) => retargeting_skeleton_mappers = Some(m),
                HkbCharacterSetupVisitor::AnimationSkeleton(m) => animation_skeleton = Some(m),
                HkbCharacterSetupVisitor::RagdollToAnimationSkeletonMapper(m) => ragdoll_to_animation_skeleton_mapper = Some(m),
                HkbCharacterSetupVisitor::AnimationToRagdollSkeletonMapper(m) => animation_to_ragdoll_skeleton_mapper = Some(m),
                HkbCharacterSetupVisitor::AnimationBindingSet(m) => animation_binding_set = Some(m),
                HkbCharacterSetupVisitor::Data(m) => data = Some(m),
                HkbCharacterSetupVisitor::MirroredSkeleton(m) => mirrored_skeleton = Some(m),
                HkbCharacterSetupVisitor::CharacterPropertyIdMap(m) => character_property_id_map = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            retargeting_skeleton_mappers: retargeting_skeleton_mappers.unwrap_or_default(),
            animation_skeleton: animation_skeleton.unwrap_or_default().into_inner(),
            ragdoll_to_animation_skeleton_mapper: ragdoll_to_animation_skeleton_mapper.unwrap_or_default().into_inner(),
            animation_to_ragdoll_skeleton_mapper: animation_to_ragdoll_skeleton_mapper.unwrap_or_default().into_inner(),
            animation_binding_set: animation_binding_set.unwrap_or_default().into_inner(),
            data: data.unwrap_or_default().into_inner(),
            mirrored_skeleton: mirrored_skeleton.unwrap_or_default().into_inner(),
            character_property_id_map: character_property_id_map.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbCharacterSetup<'a>> for Vec<HkbCharacterSetupVisitor<'a>> {
    fn from(data: &HkbCharacterSetup<'a>) -> Self {
        vec![
            HkbCharacterSetupVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbCharacterSetupVisitor::ReferenceCount(data.reference_count.into()),
            HkbCharacterSetupVisitor::RetargetingSkeletonMappers(data.retargeting_skeleton_mappers.clone()),
            HkbCharacterSetupVisitor::AnimationSkeleton(data.animation_skeleton.clone().into()),
            HkbCharacterSetupVisitor::RagdollToAnimationSkeletonMapper(data.ragdoll_to_animation_skeleton_mapper.clone().into()),
            HkbCharacterSetupVisitor::AnimationToRagdollSkeletonMapper(data.animation_to_ragdoll_skeleton_mapper.clone().into()),
            HkbCharacterSetupVisitor::AnimationBindingSet(data.animation_binding_set.clone().into()),
            HkbCharacterSetupVisitor::Data(data.data.clone().into()),
            HkbCharacterSetupVisitor::MirroredSkeleton(data.mirrored_skeleton.clone().into()),
            HkbCharacterSetupVisitor::CharacterPropertyIdMap(data.character_property_id_map.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbCharacterSetup<'de> {
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
enum HkbCharacterSetupVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "retargetingSkeletonMappers")]
    RetargetingSkeletonMappers(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "animationSkeleton")]
    AnimationSkeleton(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "ragdollToAnimationSkeletonMapper")]
    RagdollToAnimationSkeletonMapper(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "animationToRagdollSkeletonMapper")]
    AnimationToRagdollSkeletonMapper(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "animationBindingSet", skip_serializing)]
    AnimationBindingSet(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "data")]
    Data(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "mirroredSkeleton", skip_serializing)]
    MirroredSkeleton(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "characterPropertyIdMap", skip_serializing)]
    CharacterPropertyIdMap(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterSetupVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("retargetingSkeletonMappers" => RetargetingSkeletonMappers(HkArrayRef<Cow<'de, str>>)),
    ("animationSkeleton" => AnimationSkeleton(Primitive<Cow<'de, str>>)),
    ("ragdollToAnimationSkeletonMapper" => RagdollToAnimationSkeletonMapper(Primitive<Cow<'de, str>>)),
    ("animationToRagdollSkeletonMapper" => AnimationToRagdollSkeletonMapper(Primitive<Cow<'de, str>>)),
    ("animationBindingSet" => AnimationBindingSet(Primitive<Cow<'de, str>>)),
    ("data" => Data(Primitive<Cow<'de, str>>)),
    ("mirroredSkeleton" => MirroredSkeleton(Primitive<Cow<'de, str>>)),
    ("characterPropertyIdMap" => CharacterPropertyIdMap(Primitive<Cow<'de, str>>)),
}
