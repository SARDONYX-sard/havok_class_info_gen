//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbCharacterStringData`
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

/// `hkbCharacterStringData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 132
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x655b42bc`
/// -   version: 5
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbCharacterStringData<'a> {
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
    /// -   name:`"deformableSkinNames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub deformable_skin_names: HkArrayStringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"rigidSkinNames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub rigid_skin_names: HkArrayStringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"animationNames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub animation_names: HkArrayStringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"animationFilenames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub animation_filenames: HkArrayStringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"characterPropertyNames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub character_property_names: HkArrayStringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"retargetingSkeletonMapperFilenames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub retargeting_skeleton_mapper_filenames: HkArrayStringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"lodNames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub lod_names: HkArrayStringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"mirroredSyncPointSubstringsA"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    pub mirrored_sync_point_substrings_a: HkArrayStringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"mirroredSyncPointSubstringsB"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    pub mirrored_sync_point_substrings_b: HkArrayStringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"rigName"`
    /// -   type: `hkStringPtr`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    pub rig_name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"ragdollName"`
    /// -   type: `hkStringPtr`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    pub ragdoll_name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"behaviorFilename"`
    /// -   type: `hkStringPtr`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    pub behavior_filename: Cow<'a, str>,
}

impl Serialize for HkbCharacterStringData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbCharacterStringDataVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbCharacterStringData<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbCharacterStringDataVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbCharacterStringDataVisitor<'a>>> for HkbCharacterStringData<'a> {
    fn from(_values: Vec<HkbCharacterStringDataVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut deformable_skin_names = None;
            let mut rigid_skin_names = None;
            let mut animation_names = None;
            let mut animation_filenames = None;
            let mut character_property_names = None;
            let mut retargeting_skeleton_mapper_filenames = None;
            let mut lod_names = None;
            let mut mirrored_sync_point_substrings_a = None;
            let mut mirrored_sync_point_substrings_b = None;
            let mut name = None;
            let mut rig_name = None;
            let mut ragdoll_name = None;
            let mut behavior_filename = None;


        for _value in _values {
            match _value {
                HkbCharacterStringDataVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbCharacterStringDataVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbCharacterStringDataVisitor::DeformableSkinNames(m) => deformable_skin_names = Some(m),
                HkbCharacterStringDataVisitor::RigidSkinNames(m) => rigid_skin_names = Some(m),
                HkbCharacterStringDataVisitor::AnimationNames(m) => animation_names = Some(m),
                HkbCharacterStringDataVisitor::AnimationFilenames(m) => animation_filenames = Some(m),
                HkbCharacterStringDataVisitor::CharacterPropertyNames(m) => character_property_names = Some(m),
                HkbCharacterStringDataVisitor::RetargetingSkeletonMapperFilenames(m) => retargeting_skeleton_mapper_filenames = Some(m),
                HkbCharacterStringDataVisitor::LodNames(m) => lod_names = Some(m),
                HkbCharacterStringDataVisitor::MirroredSyncPointSubstringsA(m) => mirrored_sync_point_substrings_a = Some(m),
                HkbCharacterStringDataVisitor::MirroredSyncPointSubstringsB(m) => mirrored_sync_point_substrings_b = Some(m),
                HkbCharacterStringDataVisitor::Name(m) => name = Some(m),
                HkbCharacterStringDataVisitor::RigName(m) => rig_name = Some(m),
                HkbCharacterStringDataVisitor::RagdollName(m) => ragdoll_name = Some(m),
                HkbCharacterStringDataVisitor::BehaviorFilename(m) => behavior_filename = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            deformable_skin_names: deformable_skin_names.unwrap_or_default(),
            rigid_skin_names: rigid_skin_names.unwrap_or_default(),
            animation_names: animation_names.unwrap_or_default(),
            animation_filenames: animation_filenames.unwrap_or_default(),
            character_property_names: character_property_names.unwrap_or_default(),
            retargeting_skeleton_mapper_filenames: retargeting_skeleton_mapper_filenames.unwrap_or_default(),
            lod_names: lod_names.unwrap_or_default(),
            mirrored_sync_point_substrings_a: mirrored_sync_point_substrings_a.unwrap_or_default(),
            mirrored_sync_point_substrings_b: mirrored_sync_point_substrings_b.unwrap_or_default(),
            name: name.unwrap_or_default().into_inner(),
            rig_name: rig_name.unwrap_or_default().into_inner(),
            ragdoll_name: ragdoll_name.unwrap_or_default().into_inner(),
            behavior_filename: behavior_filename.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbCharacterStringData<'a>> for Vec<HkbCharacterStringDataVisitor<'a>> {
    fn from(data: &HkbCharacterStringData<'a>) -> Self {
        vec![
            HkbCharacterStringDataVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbCharacterStringDataVisitor::ReferenceCount(data.reference_count.into()),
            HkbCharacterStringDataVisitor::DeformableSkinNames(data.deformable_skin_names.clone()),
            HkbCharacterStringDataVisitor::RigidSkinNames(data.rigid_skin_names.clone()),
            HkbCharacterStringDataVisitor::AnimationNames(data.animation_names.clone()),
            HkbCharacterStringDataVisitor::AnimationFilenames(data.animation_filenames.clone()),
            HkbCharacterStringDataVisitor::CharacterPropertyNames(data.character_property_names.clone()),
            HkbCharacterStringDataVisitor::RetargetingSkeletonMapperFilenames(data.retargeting_skeleton_mapper_filenames.clone()),
            HkbCharacterStringDataVisitor::LodNames(data.lod_names.clone()),
            HkbCharacterStringDataVisitor::MirroredSyncPointSubstringsA(data.mirrored_sync_point_substrings_a.clone()),
            HkbCharacterStringDataVisitor::MirroredSyncPointSubstringsB(data.mirrored_sync_point_substrings_b.clone()),
            HkbCharacterStringDataVisitor::Name(data.name.clone().into()),
            HkbCharacterStringDataVisitor::RigName(data.rig_name.clone().into()),
            HkbCharacterStringDataVisitor::RagdollName(data.ragdoll_name.clone().into()),
            HkbCharacterStringDataVisitor::BehaviorFilename(data.behavior_filename.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbCharacterStringData<'de> {
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
enum HkbCharacterStringDataVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "deformableSkinNames")]
    DeformableSkinNames(HkArrayStringPtr<'a>),
    /// Visitor fields
    #[serde(rename = "rigidSkinNames")]
    RigidSkinNames(HkArrayStringPtr<'a>),
    /// Visitor fields
    #[serde(rename = "animationNames")]
    AnimationNames(HkArrayStringPtr<'a>),
    /// Visitor fields
    #[serde(rename = "animationFilenames")]
    AnimationFilenames(HkArrayStringPtr<'a>),
    /// Visitor fields
    #[serde(rename = "characterPropertyNames")]
    CharacterPropertyNames(HkArrayStringPtr<'a>),
    /// Visitor fields
    #[serde(rename = "retargetingSkeletonMapperFilenames")]
    RetargetingSkeletonMapperFilenames(HkArrayStringPtr<'a>),
    /// Visitor fields
    #[serde(rename = "lodNames")]
    LodNames(HkArrayStringPtr<'a>),
    /// Visitor fields
    #[serde(rename = "mirroredSyncPointSubstringsA")]
    MirroredSyncPointSubstringsA(HkArrayStringPtr<'a>),
    /// Visitor fields
    #[serde(rename = "mirroredSyncPointSubstringsB")]
    MirroredSyncPointSubstringsB(HkArrayStringPtr<'a>),
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "rigName")]
    RigName(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "ragdollName")]
    RagdollName(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "behaviorFilename")]
    BehaviorFilename(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterStringDataVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("deformableSkinNames" => DeformableSkinNames(HkArrayStringPtr<'de>)),
    ("rigidSkinNames" => RigidSkinNames(HkArrayStringPtr<'de>)),
    ("animationNames" => AnimationNames(HkArrayStringPtr<'de>)),
    ("animationFilenames" => AnimationFilenames(HkArrayStringPtr<'de>)),
    ("characterPropertyNames" => CharacterPropertyNames(HkArrayStringPtr<'de>)),
    ("retargetingSkeletonMapperFilenames" => RetargetingSkeletonMapperFilenames(HkArrayStringPtr<'de>)),
    ("lodNames" => LodNames(HkArrayStringPtr<'de>)),
    ("mirroredSyncPointSubstringsA" => MirroredSyncPointSubstringsA(HkArrayStringPtr<'de>)),
    ("mirroredSyncPointSubstringsB" => MirroredSyncPointSubstringsB(HkArrayStringPtr<'de>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("rigName" => RigName(Primitive<Cow<'de, str>>)),
    ("ragdollName" => RagdollName(Primitive<Cow<'de, str>>)),
    ("behaviorFilename" => BehaviorFilename(Primitive<Cow<'de, str>>)),
}
