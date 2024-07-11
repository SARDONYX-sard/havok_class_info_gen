//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbClientCharacterState`
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

/// `hkbClientCharacterState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 208
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xa2624c97`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbClientCharacterState<'a> {
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
    /// -   name:`"deformableSkinIds"`
    /// -   type: `hkArray<hkUint64>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub deformable_skin_ids: HkArrayNum<u64>,
    /// # C++ Class Fields Info
    /// -   name:`"rigidSkinIds"`
    /// -   type: `hkArray<hkUint64>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub rigid_skin_ids: HkArrayNum<u64>,
    /// # C++ Class Fields Info
    /// -   name:`"externalEventIds"`
    /// -   type: `hkArray<hkInt16>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub external_event_ids: HkArrayNum<i16>,
    /// # C++ Class Fields Info
    /// -   name:`"auxiliaryInfo"`
    /// -   type: `hkArray<hkbAuxiliaryNodeInfo*>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub auxiliary_info: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"activeEventIds"`
    /// -   type: `hkArray<hkInt16>`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub active_event_ids: HkArrayNum<i16>,
    /// # C++ Class Fields Info
    /// -   name:`"activeVariableIds"`
    /// -   type: `hkArray<hkInt16>`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub active_variable_ids: HkArrayNum<i16>,
    /// # C++ Class Fields Info
    /// -   name:`"characterId"`
    /// -   type: `hkUint64`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub character_id: u64,
    /// # C++ Class Fields Info
    /// -   name:`"instanceName"`
    /// -   type: `hkStringPtr`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    pub instance_name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"templateName"`
    /// -   type: `hkStringPtr`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    pub template_name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"fullPathToProject"`
    /// -   type: `hkStringPtr`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub full_path_to_project: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"behaviorData"`
    /// -   type: `struct hkbBehaviorGraphData*`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    pub behavior_data: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"behaviorInternalState"`
    /// -   type: `struct hkbBehaviorGraphInternalState*`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    pub behavior_internal_state: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"nodeIdToInternalStateMap"`
    /// -   type: `void*`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub node_id_to_internal_state_map: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"visible"`
    /// -   type: `hkBool`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    pub visible: bool,
    /// # C++ Class Fields Info
    /// -   name:`"elapsedSimulationTime"`
    /// -   type: `hkReal`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    pub elapsed_simulation_time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"skeleton"`
    /// -   type: `struct hkaSkeleton*`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    pub skeleton: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"worldFromModel"`
    /// -   type: `hkQsTransform`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    pub world_from_model: QsTransform<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"poseModelSpace"`
    /// -   type: `hkArray<hkQsTransform>`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    pub pose_model_space: HkArrayMatrix3<QsTransform<f32>>,
    /// # C++ Class Fields Info
    /// -   name:`"rigidAttachmentTransforms"`
    /// -   type: `hkArray<hkQsTransform>`
    /// - offset: 188
    /// -  flags: `FLAGS_NONE`
    pub rigid_attachment_transforms: HkArrayMatrix3<QsTransform<f32>>,
}

impl Serialize for HkbClientCharacterState<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbClientCharacterStateVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbClientCharacterState<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbClientCharacterStateVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbClientCharacterStateVisitor<'a>>> for HkbClientCharacterState<'a> {
    fn from(_values: Vec<HkbClientCharacterStateVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut deformable_skin_ids = None;
            let mut rigid_skin_ids = None;
            let mut external_event_ids = None;
            let mut auxiliary_info = None;
            let mut active_event_ids = None;
            let mut active_variable_ids = None;
            let mut character_id = None;
            let mut instance_name = None;
            let mut template_name = None;
            let mut full_path_to_project = None;
            let mut behavior_data = None;
            let mut behavior_internal_state = None;
            let mut node_id_to_internal_state_map = None;
            let mut visible = None;
            let mut elapsed_simulation_time = None;
            let mut skeleton = None;
            let mut world_from_model = None;
            let mut pose_model_space = None;
            let mut rigid_attachment_transforms = None;


        for _value in _values {
            match _value {
                HkbClientCharacterStateVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbClientCharacterStateVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbClientCharacterStateVisitor::DeformableSkinIds(m) => deformable_skin_ids = Some(m),
                HkbClientCharacterStateVisitor::RigidSkinIds(m) => rigid_skin_ids = Some(m),
                HkbClientCharacterStateVisitor::ExternalEventIds(m) => external_event_ids = Some(m),
                HkbClientCharacterStateVisitor::AuxiliaryInfo(m) => auxiliary_info = Some(m),
                HkbClientCharacterStateVisitor::ActiveEventIds(m) => active_event_ids = Some(m),
                HkbClientCharacterStateVisitor::ActiveVariableIds(m) => active_variable_ids = Some(m),
                HkbClientCharacterStateVisitor::CharacterId(m) => character_id = Some(m),
                HkbClientCharacterStateVisitor::InstanceName(m) => instance_name = Some(m),
                HkbClientCharacterStateVisitor::TemplateName(m) => template_name = Some(m),
                HkbClientCharacterStateVisitor::FullPathToProject(m) => full_path_to_project = Some(m),
                HkbClientCharacterStateVisitor::BehaviorData(m) => behavior_data = Some(m),
                HkbClientCharacterStateVisitor::BehaviorInternalState(m) => behavior_internal_state = Some(m),
                HkbClientCharacterStateVisitor::NodeIdToInternalStateMap(m) => node_id_to_internal_state_map = Some(m),
                HkbClientCharacterStateVisitor::Visible(m) => visible = Some(m),
                HkbClientCharacterStateVisitor::ElapsedSimulationTime(m) => elapsed_simulation_time = Some(m),
                HkbClientCharacterStateVisitor::Skeleton(m) => skeleton = Some(m),
                HkbClientCharacterStateVisitor::WorldFromModel(m) => world_from_model = Some(m),
                HkbClientCharacterStateVisitor::PoseModelSpace(m) => pose_model_space = Some(m),
                HkbClientCharacterStateVisitor::RigidAttachmentTransforms(m) => rigid_attachment_transforms = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            deformable_skin_ids: deformable_skin_ids.unwrap_or_default(),
            rigid_skin_ids: rigid_skin_ids.unwrap_or_default(),
            external_event_ids: external_event_ids.unwrap_or_default(),
            auxiliary_info: auxiliary_info.unwrap_or_default(),
            active_event_ids: active_event_ids.unwrap_or_default(),
            active_variable_ids: active_variable_ids.unwrap_or_default(),
            character_id: character_id.unwrap_or_default().into_inner(),
            instance_name: instance_name.unwrap_or_default().into_inner(),
            template_name: template_name.unwrap_or_default().into_inner(),
            full_path_to_project: full_path_to_project.unwrap_or_default().into_inner(),
            behavior_data: behavior_data.unwrap_or_default().into_inner(),
            behavior_internal_state: behavior_internal_state.unwrap_or_default().into_inner(),
            node_id_to_internal_state_map: node_id_to_internal_state_map.unwrap_or_default().into_inner(),
            visible: visible.unwrap_or_default().into_inner(),
            elapsed_simulation_time: elapsed_simulation_time.unwrap_or_default().into_inner(),
            skeleton: skeleton.unwrap_or_default().into_inner(),
            world_from_model: world_from_model.unwrap_or_default().into_inner(),
            pose_model_space: pose_model_space.unwrap_or_default(),
            rigid_attachment_transforms: rigid_attachment_transforms.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbClientCharacterState<'a>> for Vec<HkbClientCharacterStateVisitor<'a>> {
    fn from(data: &HkbClientCharacterState<'a>) -> Self {
        vec![
            HkbClientCharacterStateVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbClientCharacterStateVisitor::ReferenceCount(data.reference_count.into()),
            HkbClientCharacterStateVisitor::DeformableSkinIds(data.deformable_skin_ids.clone()),
            HkbClientCharacterStateVisitor::RigidSkinIds(data.rigid_skin_ids.clone()),
            HkbClientCharacterStateVisitor::ExternalEventIds(data.external_event_ids.clone()),
            HkbClientCharacterStateVisitor::AuxiliaryInfo(data.auxiliary_info.clone()),
            HkbClientCharacterStateVisitor::ActiveEventIds(data.active_event_ids.clone()),
            HkbClientCharacterStateVisitor::ActiveVariableIds(data.active_variable_ids.clone()),
            HkbClientCharacterStateVisitor::CharacterId(data.character_id.into()),
            HkbClientCharacterStateVisitor::InstanceName(data.instance_name.clone().into()),
            HkbClientCharacterStateVisitor::TemplateName(data.template_name.clone().into()),
            HkbClientCharacterStateVisitor::FullPathToProject(data.full_path_to_project.clone().into()),
            HkbClientCharacterStateVisitor::BehaviorData(data.behavior_data.clone().into()),
            HkbClientCharacterStateVisitor::BehaviorInternalState(data.behavior_internal_state.clone().into()),
            HkbClientCharacterStateVisitor::NodeIdToInternalStateMap(data.node_id_to_internal_state_map.clone().into()),
            HkbClientCharacterStateVisitor::Visible(data.visible.into()),
            HkbClientCharacterStateVisitor::ElapsedSimulationTime(data.elapsed_simulation_time.into()),
            HkbClientCharacterStateVisitor::Skeleton(data.skeleton.clone().into()),
            HkbClientCharacterStateVisitor::WorldFromModel(data.world_from_model.clone().into()),
            HkbClientCharacterStateVisitor::PoseModelSpace(data.pose_model_space.clone()),
            HkbClientCharacterStateVisitor::RigidAttachmentTransforms(data.rigid_attachment_transforms.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbClientCharacterState<'de> {
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
enum HkbClientCharacterStateVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "deformableSkinIds")]
    DeformableSkinIds(HkArrayNum<u64>),
    /// Visitor fields
    #[serde(rename = "rigidSkinIds")]
    RigidSkinIds(HkArrayNum<u64>),
    /// Visitor fields
    #[serde(rename = "externalEventIds")]
    ExternalEventIds(HkArrayNum<i16>),
    /// Visitor fields
    #[serde(rename = "auxiliaryInfo")]
    AuxiliaryInfo(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "activeEventIds")]
    ActiveEventIds(HkArrayNum<i16>),
    /// Visitor fields
    #[serde(rename = "activeVariableIds")]
    ActiveVariableIds(HkArrayNum<i16>),
    /// Visitor fields
    #[serde(rename = "characterId")]
    CharacterId(Primitive<u64>),
    /// Visitor fields
    #[serde(rename = "instanceName")]
    InstanceName(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "templateName")]
    TemplateName(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "fullPathToProject")]
    FullPathToProject(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "behaviorData")]
    BehaviorData(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "behaviorInternalState")]
    BehaviorInternalState(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "nodeIdToInternalStateMap", skip_serializing)]
    NodeIdToInternalStateMap(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "visible")]
    Visible(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "elapsedSimulationTime")]
    ElapsedSimulationTime(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "skeleton")]
    Skeleton(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "worldFromModel")]
    WorldFromModel(Primitive<QsTransform<f32>>),
    /// Visitor fields
    #[serde(rename = "poseModelSpace")]
    PoseModelSpace(HkArrayMatrix3<QsTransform<f32>>),
    /// Visitor fields
    #[serde(rename = "rigidAttachmentTransforms")]
    RigidAttachmentTransforms(HkArrayMatrix3<QsTransform<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbClientCharacterStateVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("deformableSkinIds" => DeformableSkinIds(HkArrayNum<u64>)),
    ("rigidSkinIds" => RigidSkinIds(HkArrayNum<u64>)),
    ("externalEventIds" => ExternalEventIds(HkArrayNum<i16>)),
    ("auxiliaryInfo" => AuxiliaryInfo(HkArrayRef<Cow<'de, str>>)),
    ("activeEventIds" => ActiveEventIds(HkArrayNum<i16>)),
    ("activeVariableIds" => ActiveVariableIds(HkArrayNum<i16>)),
    ("characterId" => CharacterId(Primitive<u64>)),
    ("instanceName" => InstanceName(Primitive<Cow<'de, str>>)),
    ("templateName" => TemplateName(Primitive<Cow<'de, str>>)),
    ("fullPathToProject" => FullPathToProject(Primitive<Cow<'de, str>>)),
    ("behaviorData" => BehaviorData(Primitive<Cow<'de, str>>)),
    ("behaviorInternalState" => BehaviorInternalState(Primitive<Cow<'de, str>>)),
    ("nodeIdToInternalStateMap" => NodeIdToInternalStateMap(Primitive<Cow<'de, str>>)),
    ("visible" => Visible(Primitive<bool>)),
    ("elapsedSimulationTime" => ElapsedSimulationTime(Primitive<f32>)),
    ("skeleton" => Skeleton(Primitive<Cow<'de, str>>)),
    ("worldFromModel" => WorldFromModel(Primitive<QsTransform<f32>>)),
    ("poseModelSpace" => PoseModelSpace(HkArrayMatrix3<QsTransform<f32>>)),
    ("rigidAttachmentTransforms" => RigidAttachmentTransforms(HkArrayMatrix3<QsTransform<f32>>)),
}
