//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbCharacterAddedInfo`
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

/// `hkbCharacterAddedInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x3544e182`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbCharacterAddedInfo<'a> {
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
    /// -   name:`"characterId"`
    /// -   type: `hkUint64`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub character_id: u64,
    /// # C++ Class Fields Info
    /// -   name:`"instanceName"`
    /// -   type: `hkStringPtr`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub instance_name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"templateName"`
    /// -   type: `hkStringPtr`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub template_name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"fullPathToProject"`
    /// -   type: `hkStringPtr`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub full_path_to_project: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"skeleton"`
    /// -   type: `struct hkaSkeleton*`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub skeleton: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"worldFromModel"`
    /// -   type: `hkQsTransform`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub world_from_model: QsTransform<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"poseModelSpace"`
    /// -   type: `hkArray<hkQsTransform>`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub pose_model_space: HkArrayMatrix3<QsTransform<f32>>,
}

impl Serialize for HkbCharacterAddedInfo<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbCharacterAddedInfoVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbCharacterAddedInfo<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbCharacterAddedInfoVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbCharacterAddedInfoVisitor<'a>>> for HkbCharacterAddedInfo<'a> {
    fn from(_values: Vec<HkbCharacterAddedInfoVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut character_id = None;
            let mut instance_name = None;
            let mut template_name = None;
            let mut full_path_to_project = None;
            let mut skeleton = None;
            let mut world_from_model = None;
            let mut pose_model_space = None;


        for _value in _values {
            match _value {
                HkbCharacterAddedInfoVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbCharacterAddedInfoVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbCharacterAddedInfoVisitor::CharacterId(m) => character_id = Some(m),
                HkbCharacterAddedInfoVisitor::InstanceName(m) => instance_name = Some(m),
                HkbCharacterAddedInfoVisitor::TemplateName(m) => template_name = Some(m),
                HkbCharacterAddedInfoVisitor::FullPathToProject(m) => full_path_to_project = Some(m),
                HkbCharacterAddedInfoVisitor::Skeleton(m) => skeleton = Some(m),
                HkbCharacterAddedInfoVisitor::WorldFromModel(m) => world_from_model = Some(m),
                HkbCharacterAddedInfoVisitor::PoseModelSpace(m) => pose_model_space = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            character_id: character_id.unwrap_or_default().into_inner(),
            instance_name: instance_name.unwrap_or_default().into_inner(),
            template_name: template_name.unwrap_or_default().into_inner(),
            full_path_to_project: full_path_to_project.unwrap_or_default().into_inner(),
            skeleton: skeleton.unwrap_or_default().into_inner(),
            world_from_model: world_from_model.unwrap_or_default().into_inner(),
            pose_model_space: pose_model_space.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbCharacterAddedInfo<'a>> for Vec<HkbCharacterAddedInfoVisitor<'a>> {
    fn from(data: &HkbCharacterAddedInfo<'a>) -> Self {
        vec![
            HkbCharacterAddedInfoVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbCharacterAddedInfoVisitor::ReferenceCount(data.reference_count.into()),
            HkbCharacterAddedInfoVisitor::CharacterId(data.character_id.into()),
            HkbCharacterAddedInfoVisitor::InstanceName(data.instance_name.clone().into()),
            HkbCharacterAddedInfoVisitor::TemplateName(data.template_name.clone().into()),
            HkbCharacterAddedInfoVisitor::FullPathToProject(data.full_path_to_project.clone().into()),
            HkbCharacterAddedInfoVisitor::Skeleton(data.skeleton.clone().into()),
            HkbCharacterAddedInfoVisitor::WorldFromModel(data.world_from_model.clone().into()),
            HkbCharacterAddedInfoVisitor::PoseModelSpace(data.pose_model_space.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbCharacterAddedInfo<'de> {
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
enum HkbCharacterAddedInfoVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
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
    #[serde(rename = "skeleton")]
    Skeleton(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "worldFromModel")]
    WorldFromModel(Primitive<QsTransform<f32>>),
    /// Visitor fields
    #[serde(rename = "poseModelSpace")]
    PoseModelSpace(HkArrayMatrix3<QsTransform<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterAddedInfoVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("characterId" => CharacterId(Primitive<u64>)),
    ("instanceName" => InstanceName(Primitive<Cow<'de, str>>)),
    ("templateName" => TemplateName(Primitive<Cow<'de, str>>)),
    ("fullPathToProject" => FullPathToProject(Primitive<Cow<'de, str>>)),
    ("skeleton" => Skeleton(Primitive<Cow<'de, str>>)),
    ("worldFromModel" => WorldFromModel(Primitive<QsTransform<f32>>)),
    ("poseModelSpace" => PoseModelSpace(HkArrayMatrix3<QsTransform<f32>>)),
}
