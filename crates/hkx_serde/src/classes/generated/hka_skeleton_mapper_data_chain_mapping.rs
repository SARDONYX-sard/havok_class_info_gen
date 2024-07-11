//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaSkeletonMapperDataChainMapping`
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

/// `hkaSkeletonMapperDataChainMapping`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: false
/// - signature: `0xa528f7cf`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkaSkeletonMapperDataChainMapping {
    /// # C++ Class Fields Info
    /// -   name:`"startBoneA"`
    /// -   type: `hkInt16`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub start_bone_a: i16,
    /// # C++ Class Fields Info
    /// -   name:`"endBoneA"`
    /// -   type: `hkInt16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    pub end_bone_a: i16,
    /// # C++ Class Fields Info
    /// -   name:`"startBoneB"`
    /// -   type: `hkInt16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub start_bone_b: i16,
    /// # C++ Class Fields Info
    /// -   name:`"endBoneB"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    pub end_bone_b: i16,
    /// # C++ Class Fields Info
    /// -   name:`"startAFromBTransform"`
    /// -   type: `hkQsTransform`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub start_a_from_b_transform: QsTransform<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"endAFromBTransform"`
    /// -   type: `hkQsTransform`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub end_a_from_b_transform: QsTransform<f32>,
}

impl Serialize for HkaSkeletonMapperDataChainMapping {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkaSkeletonMapperDataChainMappingVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkaSkeletonMapperDataChainMapping {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkaSkeletonMapperDataChainMappingVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkaSkeletonMapperDataChainMappingVisitor>> for HkaSkeletonMapperDataChainMapping {
    fn from(_values: Vec<HkaSkeletonMapperDataChainMappingVisitor>) -> Self {
            let mut start_bone_a = None;
            let mut end_bone_a = None;
            let mut start_bone_b = None;
            let mut end_bone_b = None;
            let mut start_a_from_b_transform = None;
            let mut end_a_from_b_transform = None;


        for _value in _values {
            match _value {
                HkaSkeletonMapperDataChainMappingVisitor::StartBoneA(m) => start_bone_a = Some(m),
                HkaSkeletonMapperDataChainMappingVisitor::EndBoneA(m) => end_bone_a = Some(m),
                HkaSkeletonMapperDataChainMappingVisitor::StartBoneB(m) => start_bone_b = Some(m),
                HkaSkeletonMapperDataChainMappingVisitor::EndBoneB(m) => end_bone_b = Some(m),
                HkaSkeletonMapperDataChainMappingVisitor::StartAFromBTransform(m) => start_a_from_b_transform = Some(m),
                HkaSkeletonMapperDataChainMappingVisitor::EndAFromBTransform(m) => end_a_from_b_transform = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            start_bone_a: start_bone_a.unwrap_or_default().into_inner(),
            end_bone_a: end_bone_a.unwrap_or_default().into_inner(),
            start_bone_b: start_bone_b.unwrap_or_default().into_inner(),
            end_bone_b: end_bone_b.unwrap_or_default().into_inner(),
            start_a_from_b_transform: start_a_from_b_transform.unwrap_or_default().into_inner(),
            end_a_from_b_transform: end_a_from_b_transform.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkaSkeletonMapperDataChainMapping> for Vec<HkaSkeletonMapperDataChainMappingVisitor> {
    fn from(data: &HkaSkeletonMapperDataChainMapping) -> Self {
        vec![
            HkaSkeletonMapperDataChainMappingVisitor::StartBoneA(data.start_bone_a.into()),
            HkaSkeletonMapperDataChainMappingVisitor::EndBoneA(data.end_bone_a.into()),
            HkaSkeletonMapperDataChainMappingVisitor::StartBoneB(data.start_bone_b.into()),
            HkaSkeletonMapperDataChainMappingVisitor::EndBoneB(data.end_bone_b.into()),
            HkaSkeletonMapperDataChainMappingVisitor::StartAFromBTransform(data.start_a_from_b_transform.clone().into()),
            HkaSkeletonMapperDataChainMappingVisitor::EndAFromBTransform(data.end_a_from_b_transform.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkaSkeletonMapperDataChainMapping {
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
enum HkaSkeletonMapperDataChainMappingVisitor {
    /// Visitor fields
    #[serde(rename = "startBoneA")]
    StartBoneA(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "endBoneA")]
    EndBoneA(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "startBoneB")]
    StartBoneB(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "endBoneB")]
    EndBoneB(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "startAFromBTransform")]
    StartAFromBTransform(Primitive<QsTransform<f32>>),
    /// Visitor fields
    #[serde(rename = "endAFromBTransform")]
    EndAFromBTransform(Primitive<QsTransform<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaSkeletonMapperDataChainMappingVisitor, "@name",
    ("startBoneA" => StartBoneA(Primitive<i16>)),
    ("endBoneA" => EndBoneA(Primitive<i16>)),
    ("startBoneB" => StartBoneB(Primitive<i16>)),
    ("endBoneB" => EndBoneB(Primitive<i16>)),
    ("startAFromBTransform" => StartAFromBTransform(Primitive<QsTransform<f32>>)),
    ("endAFromBTransform" => EndAFromBTransform(Primitive<QsTransform<f32>>)),
}
