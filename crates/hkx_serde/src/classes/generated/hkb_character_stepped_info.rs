//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbCharacterSteppedInfo`
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

/// `hkbCharacterSteppedInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x2eda84f8`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbCharacterSteppedInfo {
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
    /// -   name:`"deltaTime"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub delta_time: f32,
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
    /// # C++ Class Fields Info
    /// -   name:`"rigidAttachmentTransforms"`
    /// -   type: `hkArray<hkQsTransform>`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    pub rigid_attachment_transforms: HkArrayMatrix3<QsTransform<f32>>,
}

impl Serialize for HkbCharacterSteppedInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbCharacterSteppedInfoVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbCharacterSteppedInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbCharacterSteppedInfoVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbCharacterSteppedInfoVisitor>> for HkbCharacterSteppedInfo {
    fn from(_values: Vec<HkbCharacterSteppedInfoVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut character_id = None;
            let mut delta_time = None;
            let mut world_from_model = None;
            let mut pose_model_space = None;
            let mut rigid_attachment_transforms = None;


        for _value in _values {
            match _value {
                HkbCharacterSteppedInfoVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbCharacterSteppedInfoVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbCharacterSteppedInfoVisitor::CharacterId(m) => character_id = Some(m),
                HkbCharacterSteppedInfoVisitor::DeltaTime(m) => delta_time = Some(m),
                HkbCharacterSteppedInfoVisitor::WorldFromModel(m) => world_from_model = Some(m),
                HkbCharacterSteppedInfoVisitor::PoseModelSpace(m) => pose_model_space = Some(m),
                HkbCharacterSteppedInfoVisitor::RigidAttachmentTransforms(m) => rigid_attachment_transforms = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            character_id: character_id.unwrap_or_default().into_inner(),
            delta_time: delta_time.unwrap_or_default().into_inner(),
            world_from_model: world_from_model.unwrap_or_default().into_inner(),
            pose_model_space: pose_model_space.unwrap_or_default(),
            rigid_attachment_transforms: rigid_attachment_transforms.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbCharacterSteppedInfo> for Vec<HkbCharacterSteppedInfoVisitor> {
    fn from(data: &HkbCharacterSteppedInfo) -> Self {
        vec![
            HkbCharacterSteppedInfoVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbCharacterSteppedInfoVisitor::ReferenceCount(data.reference_count.into()),
            HkbCharacterSteppedInfoVisitor::CharacterId(data.character_id.into()),
            HkbCharacterSteppedInfoVisitor::DeltaTime(data.delta_time.into()),
            HkbCharacterSteppedInfoVisitor::WorldFromModel(data.world_from_model.clone().into()),
            HkbCharacterSteppedInfoVisitor::PoseModelSpace(data.pose_model_space.clone()),
            HkbCharacterSteppedInfoVisitor::RigidAttachmentTransforms(data.rigid_attachment_transforms.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbCharacterSteppedInfo {
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
enum HkbCharacterSteppedInfoVisitor {
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
    #[serde(rename = "deltaTime")]
    DeltaTime(Primitive<f32>),
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
    HkbCharacterSteppedInfoVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("characterId" => CharacterId(Primitive<u64>)),
    ("deltaTime" => DeltaTime(Primitive<f32>)),
    ("worldFromModel" => WorldFromModel(Primitive<QsTransform<f32>>)),
    ("poseModelSpace" => PoseModelSpace(HkArrayMatrix3<QsTransform<f32>>)),
    ("rigidAttachmentTransforms" => RigidAttachmentTransforms(HkArrayMatrix3<QsTransform<f32>>)),
}
