//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaSkeletonLocalFrameOnBone`
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

/// `hkaSkeletonLocalFrameOnBone`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0x52e8043`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkaSkeletonLocalFrameOnBone<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"localFrame"`
    /// -   type: `struct hkLocalFrame*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub local_frame: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"boneIndex"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub bone_index: i32,
}

impl Serialize for HkaSkeletonLocalFrameOnBone<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkaSkeletonLocalFrameOnBoneVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkaSkeletonLocalFrameOnBone<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkaSkeletonLocalFrameOnBoneVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkaSkeletonLocalFrameOnBoneVisitor<'a>>> for HkaSkeletonLocalFrameOnBone<'a> {
    fn from(_values: Vec<HkaSkeletonLocalFrameOnBoneVisitor<'a>>) -> Self {
            let mut local_frame = None;
            let mut bone_index = None;


        for _value in _values {
            match _value {
                HkaSkeletonLocalFrameOnBoneVisitor::LocalFrame(m) => local_frame = Some(m),
                HkaSkeletonLocalFrameOnBoneVisitor::BoneIndex(m) => bone_index = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            local_frame: local_frame.unwrap_or_default().into_inner(),
            bone_index: bone_index.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkaSkeletonLocalFrameOnBone<'a>> for Vec<HkaSkeletonLocalFrameOnBoneVisitor<'a>> {
    fn from(data: &HkaSkeletonLocalFrameOnBone<'a>) -> Self {
        vec![
            HkaSkeletonLocalFrameOnBoneVisitor::LocalFrame(data.local_frame.clone().into()),
            HkaSkeletonLocalFrameOnBoneVisitor::BoneIndex(data.bone_index.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkaSkeletonLocalFrameOnBone<'de> {
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
enum HkaSkeletonLocalFrameOnBoneVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "localFrame")]
    LocalFrame(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "boneIndex")]
    BoneIndex(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaSkeletonLocalFrameOnBoneVisitor<'de>, "@name",
    ("localFrame" => LocalFrame(Primitive<Cow<'de, str>>)),
    ("boneIndex" => BoneIndex(Primitive<i32>)),
}
