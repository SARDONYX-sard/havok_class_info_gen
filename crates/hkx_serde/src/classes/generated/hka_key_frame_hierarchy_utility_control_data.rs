//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaKeyFrameHierarchyUtilityControlData`
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

/// `hkaKeyFrameHierarchyUtilityControlData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: false
/// - signature: `0xa3d0ac71`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkaKeyFrameHierarchyUtilityControlData {
    /// # C++ Class Fields Info
    /// -   name:`"hierarchyGain"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub hierarchy_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"velocityDamping"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub velocity_damping: f32,
    /// # C++ Class Fields Info
    /// -   name:`"accelerationGain"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub acceleration_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"velocityGain"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub velocity_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"positionGain"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub position_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"positionMaxLinearVelocity"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub position_max_linear_velocity: f32,
    /// # C++ Class Fields Info
    /// -   name:`"positionMaxAngularVelocity"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub position_max_angular_velocity: f32,
    /// # C++ Class Fields Info
    /// -   name:`"snapGain"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub snap_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"snapMaxLinearVelocity"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub snap_max_linear_velocity: f32,
    /// # C++ Class Fields Info
    /// -   name:`"snapMaxAngularVelocity"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub snap_max_angular_velocity: f32,
    /// # C++ Class Fields Info
    /// -   name:`"snapMaxLinearDistance"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub snap_max_linear_distance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"snapMaxAngularDistance"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub snap_max_angular_distance: f32,
}

impl Serialize for HkaKeyFrameHierarchyUtilityControlData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkaKeyFrameHierarchyUtilityControlDataVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkaKeyFrameHierarchyUtilityControlData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkaKeyFrameHierarchyUtilityControlDataVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkaKeyFrameHierarchyUtilityControlDataVisitor>> for HkaKeyFrameHierarchyUtilityControlData {
    fn from(_values: Vec<HkaKeyFrameHierarchyUtilityControlDataVisitor>) -> Self {
            let mut hierarchy_gain = None;
            let mut velocity_damping = None;
            let mut acceleration_gain = None;
            let mut velocity_gain = None;
            let mut position_gain = None;
            let mut position_max_linear_velocity = None;
            let mut position_max_angular_velocity = None;
            let mut snap_gain = None;
            let mut snap_max_linear_velocity = None;
            let mut snap_max_angular_velocity = None;
            let mut snap_max_linear_distance = None;
            let mut snap_max_angular_distance = None;


        for _value in _values {
            match _value {
                HkaKeyFrameHierarchyUtilityControlDataVisitor::HierarchyGain(m) => hierarchy_gain = Some(m),
                HkaKeyFrameHierarchyUtilityControlDataVisitor::VelocityDamping(m) => velocity_damping = Some(m),
                HkaKeyFrameHierarchyUtilityControlDataVisitor::AccelerationGain(m) => acceleration_gain = Some(m),
                HkaKeyFrameHierarchyUtilityControlDataVisitor::VelocityGain(m) => velocity_gain = Some(m),
                HkaKeyFrameHierarchyUtilityControlDataVisitor::PositionGain(m) => position_gain = Some(m),
                HkaKeyFrameHierarchyUtilityControlDataVisitor::PositionMaxLinearVelocity(m) => position_max_linear_velocity = Some(m),
                HkaKeyFrameHierarchyUtilityControlDataVisitor::PositionMaxAngularVelocity(m) => position_max_angular_velocity = Some(m),
                HkaKeyFrameHierarchyUtilityControlDataVisitor::SnapGain(m) => snap_gain = Some(m),
                HkaKeyFrameHierarchyUtilityControlDataVisitor::SnapMaxLinearVelocity(m) => snap_max_linear_velocity = Some(m),
                HkaKeyFrameHierarchyUtilityControlDataVisitor::SnapMaxAngularVelocity(m) => snap_max_angular_velocity = Some(m),
                HkaKeyFrameHierarchyUtilityControlDataVisitor::SnapMaxLinearDistance(m) => snap_max_linear_distance = Some(m),
                HkaKeyFrameHierarchyUtilityControlDataVisitor::SnapMaxAngularDistance(m) => snap_max_angular_distance = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            hierarchy_gain: hierarchy_gain.unwrap_or_default().into_inner(),
            velocity_damping: velocity_damping.unwrap_or_default().into_inner(),
            acceleration_gain: acceleration_gain.unwrap_or_default().into_inner(),
            velocity_gain: velocity_gain.unwrap_or_default().into_inner(),
            position_gain: position_gain.unwrap_or_default().into_inner(),
            position_max_linear_velocity: position_max_linear_velocity.unwrap_or_default().into_inner(),
            position_max_angular_velocity: position_max_angular_velocity.unwrap_or_default().into_inner(),
            snap_gain: snap_gain.unwrap_or_default().into_inner(),
            snap_max_linear_velocity: snap_max_linear_velocity.unwrap_or_default().into_inner(),
            snap_max_angular_velocity: snap_max_angular_velocity.unwrap_or_default().into_inner(),
            snap_max_linear_distance: snap_max_linear_distance.unwrap_or_default().into_inner(),
            snap_max_angular_distance: snap_max_angular_distance.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkaKeyFrameHierarchyUtilityControlData> for Vec<HkaKeyFrameHierarchyUtilityControlDataVisitor> {
    fn from(data: &HkaKeyFrameHierarchyUtilityControlData) -> Self {
        vec![
            HkaKeyFrameHierarchyUtilityControlDataVisitor::HierarchyGain(data.hierarchy_gain.into()),
            HkaKeyFrameHierarchyUtilityControlDataVisitor::VelocityDamping(data.velocity_damping.into()),
            HkaKeyFrameHierarchyUtilityControlDataVisitor::AccelerationGain(data.acceleration_gain.into()),
            HkaKeyFrameHierarchyUtilityControlDataVisitor::VelocityGain(data.velocity_gain.into()),
            HkaKeyFrameHierarchyUtilityControlDataVisitor::PositionGain(data.position_gain.into()),
            HkaKeyFrameHierarchyUtilityControlDataVisitor::PositionMaxLinearVelocity(data.position_max_linear_velocity.into()),
            HkaKeyFrameHierarchyUtilityControlDataVisitor::PositionMaxAngularVelocity(data.position_max_angular_velocity.into()),
            HkaKeyFrameHierarchyUtilityControlDataVisitor::SnapGain(data.snap_gain.into()),
            HkaKeyFrameHierarchyUtilityControlDataVisitor::SnapMaxLinearVelocity(data.snap_max_linear_velocity.into()),
            HkaKeyFrameHierarchyUtilityControlDataVisitor::SnapMaxAngularVelocity(data.snap_max_angular_velocity.into()),
            HkaKeyFrameHierarchyUtilityControlDataVisitor::SnapMaxLinearDistance(data.snap_max_linear_distance.into()),
            HkaKeyFrameHierarchyUtilityControlDataVisitor::SnapMaxAngularDistance(data.snap_max_angular_distance.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkaKeyFrameHierarchyUtilityControlData {
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
enum HkaKeyFrameHierarchyUtilityControlDataVisitor {
    /// Visitor fields
    #[serde(rename = "hierarchyGain")]
    HierarchyGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "velocityDamping")]
    VelocityDamping(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "accelerationGain")]
    AccelerationGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "velocityGain")]
    VelocityGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "positionGain")]
    PositionGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "positionMaxLinearVelocity")]
    PositionMaxLinearVelocity(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "positionMaxAngularVelocity")]
    PositionMaxAngularVelocity(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "snapGain")]
    SnapGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "snapMaxLinearVelocity")]
    SnapMaxLinearVelocity(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "snapMaxAngularVelocity")]
    SnapMaxAngularVelocity(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "snapMaxLinearDistance")]
    SnapMaxLinearDistance(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "snapMaxAngularDistance")]
    SnapMaxAngularDistance(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaKeyFrameHierarchyUtilityControlDataVisitor, "@name",
    ("hierarchyGain" => HierarchyGain(Primitive<f32>)),
    ("velocityDamping" => VelocityDamping(Primitive<f32>)),
    ("accelerationGain" => AccelerationGain(Primitive<f32>)),
    ("velocityGain" => VelocityGain(Primitive<f32>)),
    ("positionGain" => PositionGain(Primitive<f32>)),
    ("positionMaxLinearVelocity" => PositionMaxLinearVelocity(Primitive<f32>)),
    ("positionMaxAngularVelocity" => PositionMaxAngularVelocity(Primitive<f32>)),
    ("snapGain" => SnapGain(Primitive<f32>)),
    ("snapMaxLinearVelocity" => SnapMaxLinearVelocity(Primitive<f32>)),
    ("snapMaxAngularVelocity" => SnapMaxAngularVelocity(Primitive<f32>)),
    ("snapMaxLinearDistance" => SnapMaxLinearDistance(Primitive<f32>)),
    ("snapMaxAngularDistance" => SnapMaxAngularDistance(Primitive<f32>)),
}
