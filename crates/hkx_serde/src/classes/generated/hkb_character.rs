//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbCharacter`
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

/// `hkbCharacter`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 88
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x3088a5c5`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbCharacter<'a> {
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
    /// -   name:`"nearbyCharacters"`
    /// -   type: `hkArray<hkbCharacter*>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub nearby_characters: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"currentLod"`
    /// -   type: `hkInt16`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub current_lod: i16,
    /// # C++ Class Fields Info
    /// -   name:`"numTracksInLod"`
    /// -   type: `hkInt16`
    /// - offset: 22
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub num_tracks_in_lod: i16,
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"ragdollDriver"`
    /// -   type: `void*`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub ragdoll_driver: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"characterControllerDriver"`
    /// -   type: `void*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub character_controller_driver: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"footIkDriver"`
    /// -   type: `void*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub foot_ik_driver: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"handIkDriver"`
    /// -   type: `void*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub hand_ik_driver: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"setup"`
    /// -   type: `struct hkbCharacterSetup*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub setup: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"behaviorGraph"`
    /// -   type: `struct hkbBehaviorGraph*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub behavior_graph: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"projectData"`
    /// -   type: `struct hkbProjectData*`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub project_data: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"animationBindingSet"`
    /// -   type: `void*`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub animation_binding_set: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"raycastInterface"`
    /// -   type: `void*`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub raycast_interface: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"world"`
    /// -   type: `void*`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE|NOT_OWNED|SERIALIZE_IGNORED`
    pub world: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"eventQueue"`
    /// -   type: `void*`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub event_queue: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"worldFromModel"`
    /// -   type: `void*`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub world_from_model: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"poseLocal"`
    /// -   type: `hkSimpleArray<void>`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub pose_local: HkArrayRef<()>,
    /// # C++ Class Fields Info
    /// -   name:`"deleteWorldFromModel"`
    /// -   type: `hkBool`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub delete_world_from_model: bool,
    /// # C++ Class Fields Info
    /// -   name:`"deletePoseLocal"`
    /// -   type: `hkBool`
    /// - offset: 85
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub delete_pose_local: bool,
}

impl Serialize for HkbCharacter<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbCharacterVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbCharacter<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbCharacterVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbCharacterVisitor<'a>>> for HkbCharacter<'a> {
    fn from(_values: Vec<HkbCharacterVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut nearby_characters = None;
            let mut current_lod = None;
            let mut num_tracks_in_lod = None;
            let mut name = None;
            let mut ragdoll_driver = None;
            let mut character_controller_driver = None;
            let mut foot_ik_driver = None;
            let mut hand_ik_driver = None;
            let mut setup = None;
            let mut behavior_graph = None;
            let mut project_data = None;
            let mut animation_binding_set = None;
            let mut raycast_interface = None;
            let mut world = None;
            let mut event_queue = None;
            let mut world_from_model = None;
            let mut pose_local = None;
            let mut delete_world_from_model = None;
            let mut delete_pose_local = None;


        for _value in _values {
            match _value {
                HkbCharacterVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbCharacterVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbCharacterVisitor::NearbyCharacters(m) => nearby_characters = Some(m),
                HkbCharacterVisitor::CurrentLod(m) => current_lod = Some(m),
                HkbCharacterVisitor::NumTracksInLod(m) => num_tracks_in_lod = Some(m),
                HkbCharacterVisitor::Name(m) => name = Some(m),
                HkbCharacterVisitor::RagdollDriver(m) => ragdoll_driver = Some(m),
                HkbCharacterVisitor::CharacterControllerDriver(m) => character_controller_driver = Some(m),
                HkbCharacterVisitor::FootIkDriver(m) => foot_ik_driver = Some(m),
                HkbCharacterVisitor::HandIkDriver(m) => hand_ik_driver = Some(m),
                HkbCharacterVisitor::Setup(m) => setup = Some(m),
                HkbCharacterVisitor::BehaviorGraph(m) => behavior_graph = Some(m),
                HkbCharacterVisitor::ProjectData(m) => project_data = Some(m),
                HkbCharacterVisitor::AnimationBindingSet(m) => animation_binding_set = Some(m),
                HkbCharacterVisitor::RaycastInterface(m) => raycast_interface = Some(m),
                HkbCharacterVisitor::World(m) => world = Some(m),
                HkbCharacterVisitor::EventQueue(m) => event_queue = Some(m),
                HkbCharacterVisitor::WorldFromModel(m) => world_from_model = Some(m),
                HkbCharacterVisitor::PoseLocal(m) => pose_local = Some(m),
                HkbCharacterVisitor::DeleteWorldFromModel(m) => delete_world_from_model = Some(m),
                HkbCharacterVisitor::DeletePoseLocal(m) => delete_pose_local = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            nearby_characters: nearby_characters.unwrap_or_default(),
            current_lod: current_lod.unwrap_or_default().into_inner(),
            num_tracks_in_lod: num_tracks_in_lod.unwrap_or_default().into_inner(),
            name: name.unwrap_or_default().into_inner(),
            ragdoll_driver: ragdoll_driver.unwrap_or_default().into_inner(),
            character_controller_driver: character_controller_driver.unwrap_or_default().into_inner(),
            foot_ik_driver: foot_ik_driver.unwrap_or_default().into_inner(),
            hand_ik_driver: hand_ik_driver.unwrap_or_default().into_inner(),
            setup: setup.unwrap_or_default().into_inner(),
            behavior_graph: behavior_graph.unwrap_or_default().into_inner(),
            project_data: project_data.unwrap_or_default().into_inner(),
            animation_binding_set: animation_binding_set.unwrap_or_default().into_inner(),
            raycast_interface: raycast_interface.unwrap_or_default().into_inner(),
            world: world.unwrap_or_default().into_inner(),
            event_queue: event_queue.unwrap_or_default().into_inner(),
            world_from_model: world_from_model.unwrap_or_default().into_inner(),
            pose_local: pose_local.unwrap_or_default(),
            delete_world_from_model: delete_world_from_model.unwrap_or_default().into_inner(),
            delete_pose_local: delete_pose_local.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbCharacter<'a>> for Vec<HkbCharacterVisitor<'a>> {
    fn from(data: &HkbCharacter<'a>) -> Self {
        vec![
            HkbCharacterVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbCharacterVisitor::ReferenceCount(data.reference_count.into()),
            HkbCharacterVisitor::NearbyCharacters(data.nearby_characters.clone()),
            HkbCharacterVisitor::CurrentLod(data.current_lod.into()),
            HkbCharacterVisitor::NumTracksInLod(data.num_tracks_in_lod.into()),
            HkbCharacterVisitor::Name(data.name.clone().into()),
            HkbCharacterVisitor::RagdollDriver(data.ragdoll_driver.clone().into()),
            HkbCharacterVisitor::CharacterControllerDriver(data.character_controller_driver.clone().into()),
            HkbCharacterVisitor::FootIkDriver(data.foot_ik_driver.clone().into()),
            HkbCharacterVisitor::HandIkDriver(data.hand_ik_driver.clone().into()),
            HkbCharacterVisitor::Setup(data.setup.clone().into()),
            HkbCharacterVisitor::BehaviorGraph(data.behavior_graph.clone().into()),
            HkbCharacterVisitor::ProjectData(data.project_data.clone().into()),
            HkbCharacterVisitor::AnimationBindingSet(data.animation_binding_set.clone().into()),
            HkbCharacterVisitor::RaycastInterface(data.raycast_interface.clone().into()),
            HkbCharacterVisitor::World(data.world.clone().into()),
            HkbCharacterVisitor::EventQueue(data.event_queue.clone().into()),
            HkbCharacterVisitor::WorldFromModel(data.world_from_model.clone().into()),
            HkbCharacterVisitor::PoseLocal(data.pose_local.clone()),
            HkbCharacterVisitor::DeleteWorldFromModel(data.delete_world_from_model.into()),
            HkbCharacterVisitor::DeletePoseLocal(data.delete_pose_local.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbCharacter<'de> {
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
enum HkbCharacterVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "nearbyCharacters")]
    NearbyCharacters(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "currentLod")]
    CurrentLod(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "numTracksInLod", skip_serializing)]
    NumTracksInLod(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "ragdollDriver", skip_serializing)]
    RagdollDriver(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "characterControllerDriver", skip_serializing)]
    CharacterControllerDriver(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "footIkDriver", skip_serializing)]
    FootIkDriver(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "handIkDriver", skip_serializing)]
    HandIkDriver(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "setup")]
    Setup(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "behaviorGraph")]
    BehaviorGraph(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "projectData")]
    ProjectData(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "animationBindingSet", skip_serializing)]
    AnimationBindingSet(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "raycastInterface", skip_serializing)]
    RaycastInterface(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "world", skip_serializing)]
    World(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "eventQueue", skip_serializing)]
    EventQueue(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "worldFromModel", skip_serializing)]
    WorldFromModel(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "poseLocal", skip_serializing)]
    PoseLocal(HkArrayRef<()>),
    /// Visitor fields
    #[serde(rename = "deleteWorldFromModel", skip_serializing)]
    DeleteWorldFromModel(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "deletePoseLocal", skip_serializing)]
    DeletePoseLocal(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("nearbyCharacters" => NearbyCharacters(HkArrayRef<Cow<'de, str>>)),
    ("currentLod" => CurrentLod(Primitive<i16>)),
    ("numTracksInLod" => NumTracksInLod(Primitive<i16>)),
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("ragdollDriver" => RagdollDriver(Primitive<Cow<'de, str>>)),
    ("characterControllerDriver" => CharacterControllerDriver(Primitive<Cow<'de, str>>)),
    ("footIkDriver" => FootIkDriver(Primitive<Cow<'de, str>>)),
    ("handIkDriver" => HandIkDriver(Primitive<Cow<'de, str>>)),
    ("setup" => Setup(Primitive<Cow<'de, str>>)),
    ("behaviorGraph" => BehaviorGraph(Primitive<Cow<'de, str>>)),
    ("projectData" => ProjectData(Primitive<Cow<'de, str>>)),
    ("animationBindingSet" => AnimationBindingSet(Primitive<Cow<'de, str>>)),
    ("raycastInterface" => RaycastInterface(Primitive<Cow<'de, str>>)),
    ("world" => World(Primitive<Cow<'de, str>>)),
    ("eventQueue" => EventQueue(Primitive<Cow<'de, str>>)),
    ("worldFromModel" => WorldFromModel(Primitive<Cow<'de, str>>)),
    ("poseLocal" => PoseLocal(HkArrayRef<()>)),
    ("deleteWorldFromModel" => DeleteWorldFromModel(Primitive<bool>)),
    ("deletePoseLocal" => DeletePoseLocal(Primitive<bool>)),
}
