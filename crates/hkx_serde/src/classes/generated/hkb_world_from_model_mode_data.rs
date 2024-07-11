//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbWorldFromModelModeData`
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

/// `hkbWorldFromModelModeData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0xa3af8783`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbWorldFromModelModeData {
    /// # C++ Class Fields Info
    /// -   name:`"poseMatchingBone0"`
    /// -   type: `hkInt16`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub pose_matching_bone_0: i16,
    /// # C++ Class Fields Info
    /// -   name:`"poseMatchingBone1"`
    /// -   type: `hkInt16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    pub pose_matching_bone_1: i16,
    /// # C++ Class Fields Info
    /// -   name:`"poseMatchingBone2"`
    /// -   type: `hkInt16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub pose_matching_bone_2: i16,
    /// # C++ Class Fields Info
    /// -   name:`"mode"`
    /// -   type: `enum WorldFromModelMode`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    pub mode: WorldFromModelMode,
}

impl Serialize for HkbWorldFromModelModeData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbWorldFromModelModeDataVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbWorldFromModelModeData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbWorldFromModelModeDataVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbWorldFromModelModeDataVisitor>> for HkbWorldFromModelModeData {
    fn from(_values: Vec<HkbWorldFromModelModeDataVisitor>) -> Self {
            let mut pose_matching_bone_0 = None;
            let mut pose_matching_bone_1 = None;
            let mut pose_matching_bone_2 = None;
            let mut mode = None;


        for _value in _values {
            match _value {
                HkbWorldFromModelModeDataVisitor::PoseMatchingBone0(m) => pose_matching_bone_0 = Some(m),
                HkbWorldFromModelModeDataVisitor::PoseMatchingBone1(m) => pose_matching_bone_1 = Some(m),
                HkbWorldFromModelModeDataVisitor::PoseMatchingBone2(m) => pose_matching_bone_2 = Some(m),
                HkbWorldFromModelModeDataVisitor::Mode(m) => mode = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            pose_matching_bone_0: pose_matching_bone_0.unwrap_or_default().into_inner(),
            pose_matching_bone_1: pose_matching_bone_1.unwrap_or_default().into_inner(),
            pose_matching_bone_2: pose_matching_bone_2.unwrap_or_default().into_inner(),
            mode: mode.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbWorldFromModelModeData> for Vec<HkbWorldFromModelModeDataVisitor> {
    fn from(data: &HkbWorldFromModelModeData) -> Self {
        vec![
            HkbWorldFromModelModeDataVisitor::PoseMatchingBone0(data.pose_matching_bone_0.into()),
            HkbWorldFromModelModeDataVisitor::PoseMatchingBone1(data.pose_matching_bone_1.into()),
            HkbWorldFromModelModeDataVisitor::PoseMatchingBone2(data.pose_matching_bone_2.into()),
            HkbWorldFromModelModeDataVisitor::Mode(data.mode.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbWorldFromModelModeData {
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
enum HkbWorldFromModelModeDataVisitor {
    /// Visitor fields
    #[serde(rename = "poseMatchingBone0")]
    PoseMatchingBone0(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "poseMatchingBone1")]
    PoseMatchingBone1(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "poseMatchingBone2")]
    PoseMatchingBone2(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "mode")]
    Mode(Primitive<WorldFromModelMode>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbWorldFromModelModeDataVisitor, "@name",
    ("poseMatchingBone0" => PoseMatchingBone0(Primitive<i16>)),
    ("poseMatchingBone1" => PoseMatchingBone1(Primitive<i16>)),
    ("poseMatchingBone2" => PoseMatchingBone2(Primitive<i16>)),
    ("mode" => Mode(Primitive<WorldFromModelMode>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum WorldFromModelMode {
    #[serde(rename = "WORLD_FROM_MODEL_MODE_USE_OLD")]
    #[default]
    WorldFromModelModeUseOld = 0,
    #[serde(rename = "WORLD_FROM_MODEL_MODE_USE_INPUT")]
    WorldFromModelModeUseInput = 1,
    #[serde(rename = "WORLD_FROM_MODEL_MODE_COMPUTE")]
    WorldFromModelModeCompute = 2,
    #[serde(rename = "WORLD_FROM_MODEL_MODE_NONE")]
    WorldFromModelModeNone = 3,
    #[serde(rename = "WORLD_FROM_MODEL_MODE_RAGDOLL")]
    WorldFromModelModeRagdoll = 4,
}
