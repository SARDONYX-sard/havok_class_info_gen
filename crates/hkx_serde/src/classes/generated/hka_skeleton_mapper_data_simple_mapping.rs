//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaSkeletonMapperDataSimpleMapping`
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

/// `hkaSkeletonMapperDataSimpleMapping`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: false
/// - signature: `0x3405deca`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkaSkeletonMapperDataSimpleMapping {
    /// # C++ Class Fields Info
    /// -   name:`"boneA"`
    /// -   type: `hkInt16`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub bone_a: i16,
    /// # C++ Class Fields Info
    /// -   name:`"boneB"`
    /// -   type: `hkInt16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    pub bone_b: i16,
    /// # C++ Class Fields Info
    /// -   name:`"aFromBTransform"`
    /// -   type: `hkQsTransform`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub a_from_b_transform: QsTransform<f32>,
}

impl Serialize for HkaSkeletonMapperDataSimpleMapping {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkaSkeletonMapperDataSimpleMappingVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkaSkeletonMapperDataSimpleMapping {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkaSkeletonMapperDataSimpleMappingVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkaSkeletonMapperDataSimpleMappingVisitor>> for HkaSkeletonMapperDataSimpleMapping {
    fn from(_values: Vec<HkaSkeletonMapperDataSimpleMappingVisitor>) -> Self {
            let mut bone_a = None;
            let mut bone_b = None;
            let mut a_from_b_transform = None;


        for _value in _values {
            match _value {
                HkaSkeletonMapperDataSimpleMappingVisitor::BoneA(m) => bone_a = Some(m),
                HkaSkeletonMapperDataSimpleMappingVisitor::BoneB(m) => bone_b = Some(m),
                HkaSkeletonMapperDataSimpleMappingVisitor::AFromBTransform(m) => a_from_b_transform = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            bone_a: bone_a.unwrap_or_default().into_inner(),
            bone_b: bone_b.unwrap_or_default().into_inner(),
            a_from_b_transform: a_from_b_transform.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkaSkeletonMapperDataSimpleMapping> for Vec<HkaSkeletonMapperDataSimpleMappingVisitor> {
    fn from(data: &HkaSkeletonMapperDataSimpleMapping) -> Self {
        vec![
            HkaSkeletonMapperDataSimpleMappingVisitor::BoneA(data.bone_a.into()),
            HkaSkeletonMapperDataSimpleMappingVisitor::BoneB(data.bone_b.into()),
            HkaSkeletonMapperDataSimpleMappingVisitor::AFromBTransform(data.a_from_b_transform.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkaSkeletonMapperDataSimpleMapping {
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
enum HkaSkeletonMapperDataSimpleMappingVisitor {
    /// Visitor fields
    #[serde(rename = "boneA")]
    BoneA(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "boneB")]
    BoneB(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "aFromBTransform")]
    AFromBTransform(Primitive<QsTransform<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaSkeletonMapperDataSimpleMappingVisitor, "@name",
    ("boneA" => BoneA(Primitive<i16>)),
    ("boneB" => BoneB(Primitive<i16>)),
    ("aFromBTransform" => AFromBTransform(Primitive<QsTransform<f32>>)),
}
