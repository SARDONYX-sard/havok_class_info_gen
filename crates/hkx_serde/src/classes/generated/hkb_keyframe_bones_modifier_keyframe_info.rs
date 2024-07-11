//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbKeyframeBonesModifierKeyframeInfo`
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

/// `hkbKeyframeBonesModifierKeyframeInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: false
/// - signature: `0x72deb7a6`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbKeyframeBonesModifierKeyframeInfo {
    /// # C++ Class Fields Info
    /// -   name:`"keyframedPosition"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub keyframed_position: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"keyframedRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub keyframed_rotation: Quaternion<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"boneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub bone_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"isValid"`
    /// -   type: `hkBool`
    /// - offset: 34
    /// -  flags: `FLAGS_NONE`
    pub is_valid: bool,
}

impl Serialize for HkbKeyframeBonesModifierKeyframeInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbKeyframeBonesModifierKeyframeInfoVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbKeyframeBonesModifierKeyframeInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbKeyframeBonesModifierKeyframeInfoVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbKeyframeBonesModifierKeyframeInfoVisitor>> for HkbKeyframeBonesModifierKeyframeInfo {
    fn from(_values: Vec<HkbKeyframeBonesModifierKeyframeInfoVisitor>) -> Self {
            let mut keyframed_position = None;
            let mut keyframed_rotation = None;
            let mut bone_index = None;
            let mut is_valid = None;


        for _value in _values {
            match _value {
                HkbKeyframeBonesModifierKeyframeInfoVisitor::KeyframedPosition(m) => keyframed_position = Some(m),
                HkbKeyframeBonesModifierKeyframeInfoVisitor::KeyframedRotation(m) => keyframed_rotation = Some(m),
                HkbKeyframeBonesModifierKeyframeInfoVisitor::BoneIndex(m) => bone_index = Some(m),
                HkbKeyframeBonesModifierKeyframeInfoVisitor::IsValid(m) => is_valid = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            keyframed_position: keyframed_position.unwrap_or_default().into_inner(),
            keyframed_rotation: keyframed_rotation.unwrap_or_default().into_inner(),
            bone_index: bone_index.unwrap_or_default().into_inner(),
            is_valid: is_valid.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbKeyframeBonesModifierKeyframeInfo> for Vec<HkbKeyframeBonesModifierKeyframeInfoVisitor> {
    fn from(data: &HkbKeyframeBonesModifierKeyframeInfo) -> Self {
        vec![
            HkbKeyframeBonesModifierKeyframeInfoVisitor::KeyframedPosition(data.keyframed_position.into()),
            HkbKeyframeBonesModifierKeyframeInfoVisitor::KeyframedRotation(data.keyframed_rotation.clone().into()),
            HkbKeyframeBonesModifierKeyframeInfoVisitor::BoneIndex(data.bone_index.into()),
            HkbKeyframeBonesModifierKeyframeInfoVisitor::IsValid(data.is_valid.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbKeyframeBonesModifierKeyframeInfo {
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
enum HkbKeyframeBonesModifierKeyframeInfoVisitor {
    /// Visitor fields
    #[serde(rename = "keyframedPosition")]
    KeyframedPosition(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "keyframedRotation")]
    KeyframedRotation(Primitive<Quaternion<f32>>),
    /// Visitor fields
    #[serde(rename = "boneIndex")]
    BoneIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "isValid")]
    IsValid(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbKeyframeBonesModifierKeyframeInfoVisitor, "@name",
    ("keyframedPosition" => KeyframedPosition(Primitive<Vector4<f32>>)),
    ("keyframedRotation" => KeyframedRotation(Primitive<Quaternion<f32>>)),
    ("boneIndex" => BoneIndex(Primitive<i16>)),
    ("isValid" => IsValid(Primitive<bool>)),
}
