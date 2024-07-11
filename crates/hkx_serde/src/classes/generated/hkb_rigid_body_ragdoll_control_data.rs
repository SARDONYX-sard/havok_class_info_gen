//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbRigidBodyRagdollControlData`
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

/// `hkbRigidBodyRagdollControlData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: false
/// - signature: `0x1e0bc068`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbRigidBodyRagdollControlData {
    /// # C++ Class Fields Info
    /// -   name:`"keyFrameHierarchyControlData"`
    /// -   type: `struct hkaKeyFrameHierarchyUtilityControlData`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|ALIGN16`
    pub key_frame_hierarchy_control_data: SingleClass<HkaKeyFrameHierarchyUtilityControlData>,
    /// # C++ Class Fields Info
    /// -   name:`"durationToBlend"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub duration_to_blend: f32,
}

impl Serialize for HkbRigidBodyRagdollControlData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbRigidBodyRagdollControlDataVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbRigidBodyRagdollControlData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbRigidBodyRagdollControlDataVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbRigidBodyRagdollControlDataVisitor>> for HkbRigidBodyRagdollControlData {
    fn from(_values: Vec<HkbRigidBodyRagdollControlDataVisitor>) -> Self {
            let mut key_frame_hierarchy_control_data = None;
            let mut duration_to_blend = None;


        for _value in _values {
            match _value {
                HkbRigidBodyRagdollControlDataVisitor::KeyFrameHierarchyControlData(m) => key_frame_hierarchy_control_data = Some(m),
                HkbRigidBodyRagdollControlDataVisitor::DurationToBlend(m) => duration_to_blend = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            key_frame_hierarchy_control_data: key_frame_hierarchy_control_data.unwrap_or_default(),
            duration_to_blend: duration_to_blend.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbRigidBodyRagdollControlData> for Vec<HkbRigidBodyRagdollControlDataVisitor> {
    fn from(data: &HkbRigidBodyRagdollControlData) -> Self {
        vec![
            HkbRigidBodyRagdollControlDataVisitor::KeyFrameHierarchyControlData(data.key_frame_hierarchy_control_data.clone()),
            HkbRigidBodyRagdollControlDataVisitor::DurationToBlend(data.duration_to_blend.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbRigidBodyRagdollControlData {
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
enum HkbRigidBodyRagdollControlDataVisitor {
    /// Visitor fields
    #[serde(rename = "keyFrameHierarchyControlData")]
    KeyFrameHierarchyControlData(SingleClass<HkaKeyFrameHierarchyUtilityControlData>),
    /// Visitor fields
    #[serde(rename = "durationToBlend")]
    DurationToBlend(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbRigidBodyRagdollControlDataVisitor, "@name",
    ("keyFrameHierarchyControlData" => KeyFrameHierarchyControlData(SingleClass<HkaKeyFrameHierarchyUtilityControlData>)),
    ("durationToBlend" => DurationToBlend(Primitive<f32>)),
}
