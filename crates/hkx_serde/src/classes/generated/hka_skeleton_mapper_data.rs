//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaSkeletonMapperData`
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

/// `hkaSkeletonMapperData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: false
/// - signature: `0x95687ea0`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkaSkeletonMapperData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"skeletonA"`
    /// -   type: `struct hkaSkeleton*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub skeleton_a: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"skeletonB"`
    /// -   type: `struct hkaSkeleton*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub skeleton_b: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"simpleMappings"`
    /// -   type: `hkArray<struct hkaSkeletonMapperDataSimpleMapping>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub simple_mappings: HkArrayClass<HkaSkeletonMapperDataSimpleMapping>,
    /// # C++ Class Fields Info
    /// -   name:`"chainMappings"`
    /// -   type: `hkArray<struct hkaSkeletonMapperDataChainMapping>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub chain_mappings: HkArrayClass<HkaSkeletonMapperDataChainMapping>,
    /// # C++ Class Fields Info
    /// -   name:`"unmappedBones"`
    /// -   type: `hkArray<hkInt16>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub unmapped_bones: HkArrayNum<i16>,
    /// # C++ Class Fields Info
    /// -   name:`"extractedMotionMapping"`
    /// -   type: `hkQsTransform`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub extracted_motion_mapping: QsTransform<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"keepUnmappedLocal"`
    /// -   type: `hkBool`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub keep_unmapped_local: bool,
    /// # C++ Class Fields Info
    /// -   name:`"mappingType"`
    /// -   type: `enum MappingType`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    pub mapping_type: MappingType,
}

impl Serialize for HkaSkeletonMapperData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkaSkeletonMapperDataVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkaSkeletonMapperData<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkaSkeletonMapperDataVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkaSkeletonMapperDataVisitor<'a>>> for HkaSkeletonMapperData<'a> {
    fn from(_values: Vec<HkaSkeletonMapperDataVisitor<'a>>) -> Self {
            let mut skeleton_a = None;
            let mut skeleton_b = None;
            let mut simple_mappings = None;
            let mut chain_mappings = None;
            let mut unmapped_bones = None;
            let mut extracted_motion_mapping = None;
            let mut keep_unmapped_local = None;
            let mut mapping_type = None;


        for _value in _values {
            match _value {
                HkaSkeletonMapperDataVisitor::SkeletonA(m) => skeleton_a = Some(m),
                HkaSkeletonMapperDataVisitor::SkeletonB(m) => skeleton_b = Some(m),
                HkaSkeletonMapperDataVisitor::SimpleMappings(m) => simple_mappings = Some(m),
                HkaSkeletonMapperDataVisitor::ChainMappings(m) => chain_mappings = Some(m),
                HkaSkeletonMapperDataVisitor::UnmappedBones(m) => unmapped_bones = Some(m),
                HkaSkeletonMapperDataVisitor::ExtractedMotionMapping(m) => extracted_motion_mapping = Some(m),
                HkaSkeletonMapperDataVisitor::KeepUnmappedLocal(m) => keep_unmapped_local = Some(m),
                HkaSkeletonMapperDataVisitor::MappingType(m) => mapping_type = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            skeleton_a: skeleton_a.unwrap_or_default().into_inner(),
            skeleton_b: skeleton_b.unwrap_or_default().into_inner(),
            simple_mappings: simple_mappings.unwrap_or_default(),
            chain_mappings: chain_mappings.unwrap_or_default(),
            unmapped_bones: unmapped_bones.unwrap_or_default(),
            extracted_motion_mapping: extracted_motion_mapping.unwrap_or_default().into_inner(),
            keep_unmapped_local: keep_unmapped_local.unwrap_or_default().into_inner(),
            mapping_type: mapping_type.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkaSkeletonMapperData<'a>> for Vec<HkaSkeletonMapperDataVisitor<'a>> {
    fn from(data: &HkaSkeletonMapperData<'a>) -> Self {
        vec![
            HkaSkeletonMapperDataVisitor::SkeletonA(data.skeleton_a.clone().into()),
            HkaSkeletonMapperDataVisitor::SkeletonB(data.skeleton_b.clone().into()),
            HkaSkeletonMapperDataVisitor::SimpleMappings(data.simple_mappings.clone()),
            HkaSkeletonMapperDataVisitor::ChainMappings(data.chain_mappings.clone()),
            HkaSkeletonMapperDataVisitor::UnmappedBones(data.unmapped_bones.clone()),
            HkaSkeletonMapperDataVisitor::ExtractedMotionMapping(data.extracted_motion_mapping.clone().into()),
            HkaSkeletonMapperDataVisitor::KeepUnmappedLocal(data.keep_unmapped_local.into()),
            HkaSkeletonMapperDataVisitor::MappingType(data.mapping_type.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkaSkeletonMapperData<'de> {
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
enum HkaSkeletonMapperDataVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "skeletonA")]
    SkeletonA(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "skeletonB")]
    SkeletonB(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "simpleMappings")]
    SimpleMappings(HkArrayClass<HkaSkeletonMapperDataSimpleMapping>),
    /// Visitor fields
    #[serde(rename = "chainMappings")]
    ChainMappings(HkArrayClass<HkaSkeletonMapperDataChainMapping>),
    /// Visitor fields
    #[serde(rename = "unmappedBones")]
    UnmappedBones(HkArrayNum<i16>),
    /// Visitor fields
    #[serde(rename = "extractedMotionMapping")]
    ExtractedMotionMapping(Primitive<QsTransform<f32>>),
    /// Visitor fields
    #[serde(rename = "keepUnmappedLocal")]
    KeepUnmappedLocal(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "mappingType")]
    MappingType(Primitive<MappingType>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaSkeletonMapperDataVisitor<'de>, "@name",
    ("skeletonA" => SkeletonA(Primitive<Cow<'de, str>>)),
    ("skeletonB" => SkeletonB(Primitive<Cow<'de, str>>)),
    ("simpleMappings" => SimpleMappings(HkArrayClass<HkaSkeletonMapperDataSimpleMapping>)),
    ("chainMappings" => ChainMappings(HkArrayClass<HkaSkeletonMapperDataChainMapping>)),
    ("unmappedBones" => UnmappedBones(HkArrayNum<i16>)),
    ("extractedMotionMapping" => ExtractedMotionMapping(Primitive<QsTransform<f32>>)),
    ("keepUnmappedLocal" => KeepUnmappedLocal(Primitive<bool>)),
    ("mappingType" => MappingType(Primitive<MappingType>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum MappingType {
    #[serde(rename = "HK_RAGDOLL_MAPPING")]
    #[default]
    HkRagdollMapping = 0,
    #[serde(rename = "HK_RETARGETING_MAPPING")]
    HkRetargetingMapping = 1,
}
