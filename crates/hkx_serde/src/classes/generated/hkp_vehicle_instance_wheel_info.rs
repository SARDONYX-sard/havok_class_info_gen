//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleInstanceWheelInfo`
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

/// `hkpVehicleInstanceWheelInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 224
/// -    vtable: false
/// - signature: `0x99f693f0`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpVehicleInstanceWheelInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"contactPoint"`
    /// -   type: `struct hkContactPoint`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub contact_point: SingleClass<HkContactPoint>,
    /// # C++ Class Fields Info
    /// -   name:`"contactFriction"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub contact_friction: f32,
    /// # C++ Class Fields Info
    /// -   name:`"contactBody"`
    /// -   type: `void*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub contact_body: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"contactShapeKey"`
    /// -   type: `hkUint32[8]`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub contact_shape_key: CStyleArray<[u32; 8]>,
    /// # C++ Class Fields Info
    /// -   name:`"hardPointWs"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub hard_point_ws: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"rayEndPointWs"`
    /// -   type: `hkVector4`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub ray_end_point_ws: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"currentSuspensionLength"`
    /// -   type: `hkReal`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    pub current_suspension_length: f32,
    /// # C++ Class Fields Info
    /// -   name:`"suspensionDirectionWs"`
    /// -   type: `hkVector4`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    pub suspension_direction_ws: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"spinAxisChassisSpace"`
    /// -   type: `hkVector4`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    pub spin_axis_chassis_space: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"spinAxisWs"`
    /// -   type: `hkVector4`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    pub spin_axis_ws: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"steeringOrientationChassisSpace"`
    /// -   type: `hkQuaternion`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    pub steering_orientation_chassis_space: Quaternion<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"spinVelocity"`
    /// -   type: `hkReal`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    pub spin_velocity: f32,
    /// # C++ Class Fields Info
    /// -   name:`"spinAngle"`
    /// -   type: `hkReal`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE`
    pub spin_angle: f32,
    /// # C++ Class Fields Info
    /// -   name:`"skidEnergyDensity"`
    /// -   type: `hkReal`
    /// - offset: 200
    /// -  flags: `FLAGS_NONE`
    pub skid_energy_density: f32,
    /// # C++ Class Fields Info
    /// -   name:`"sideForce"`
    /// -   type: `hkReal`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE`
    pub side_force: f32,
    /// # C++ Class Fields Info
    /// -   name:`"forwardSlipVelocity"`
    /// -   type: `hkReal`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    pub forward_slip_velocity: f32,
    /// # C++ Class Fields Info
    /// -   name:`"sideSlipVelocity"`
    /// -   type: `hkReal`
    /// - offset: 212
    /// -  flags: `FLAGS_NONE`
    pub side_slip_velocity: f32,
}

impl Serialize for HkpVehicleInstanceWheelInfo<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpVehicleInstanceWheelInfoVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpVehicleInstanceWheelInfo<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpVehicleInstanceWheelInfoVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpVehicleInstanceWheelInfoVisitor<'a>>> for HkpVehicleInstanceWheelInfo<'a> {
    fn from(_values: Vec<HkpVehicleInstanceWheelInfoVisitor<'a>>) -> Self {
            let mut contact_point = None;
            let mut contact_friction = None;
            let mut contact_body = None;
            let mut contact_shape_key = None;
            let mut hard_point_ws = None;
            let mut ray_end_point_ws = None;
            let mut current_suspension_length = None;
            let mut suspension_direction_ws = None;
            let mut spin_axis_chassis_space = None;
            let mut spin_axis_ws = None;
            let mut steering_orientation_chassis_space = None;
            let mut spin_velocity = None;
            let mut spin_angle = None;
            let mut skid_energy_density = None;
            let mut side_force = None;
            let mut forward_slip_velocity = None;
            let mut side_slip_velocity = None;


        for _value in _values {
            match _value {
                HkpVehicleInstanceWheelInfoVisitor::ContactPoint(m) => contact_point = Some(m),
                HkpVehicleInstanceWheelInfoVisitor::ContactFriction(m) => contact_friction = Some(m),
                HkpVehicleInstanceWheelInfoVisitor::ContactBody(m) => contact_body = Some(m),
                HkpVehicleInstanceWheelInfoVisitor::ContactShapeKey(m) => contact_shape_key = Some(m),
                HkpVehicleInstanceWheelInfoVisitor::HardPointWs(m) => hard_point_ws = Some(m),
                HkpVehicleInstanceWheelInfoVisitor::RayEndPointWs(m) => ray_end_point_ws = Some(m),
                HkpVehicleInstanceWheelInfoVisitor::CurrentSuspensionLength(m) => current_suspension_length = Some(m),
                HkpVehicleInstanceWheelInfoVisitor::SuspensionDirectionWs(m) => suspension_direction_ws = Some(m),
                HkpVehicleInstanceWheelInfoVisitor::SpinAxisChassisSpace(m) => spin_axis_chassis_space = Some(m),
                HkpVehicleInstanceWheelInfoVisitor::SpinAxisWs(m) => spin_axis_ws = Some(m),
                HkpVehicleInstanceWheelInfoVisitor::SteeringOrientationChassisSpace(m) => steering_orientation_chassis_space = Some(m),
                HkpVehicleInstanceWheelInfoVisitor::SpinVelocity(m) => spin_velocity = Some(m),
                HkpVehicleInstanceWheelInfoVisitor::SpinAngle(m) => spin_angle = Some(m),
                HkpVehicleInstanceWheelInfoVisitor::SkidEnergyDensity(m) => skid_energy_density = Some(m),
                HkpVehicleInstanceWheelInfoVisitor::SideForce(m) => side_force = Some(m),
                HkpVehicleInstanceWheelInfoVisitor::ForwardSlipVelocity(m) => forward_slip_velocity = Some(m),
                HkpVehicleInstanceWheelInfoVisitor::SideSlipVelocity(m) => side_slip_velocity = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            contact_point: contact_point.unwrap_or_default(),
            contact_friction: contact_friction.unwrap_or_default().into_inner(),
            contact_body: contact_body.unwrap_or_default().into_inner(),
            contact_shape_key: contact_shape_key.unwrap_or_default(),
            hard_point_ws: hard_point_ws.unwrap_or_default().into_inner(),
            ray_end_point_ws: ray_end_point_ws.unwrap_or_default().into_inner(),
            current_suspension_length: current_suspension_length.unwrap_or_default().into_inner(),
            suspension_direction_ws: suspension_direction_ws.unwrap_or_default().into_inner(),
            spin_axis_chassis_space: spin_axis_chassis_space.unwrap_or_default().into_inner(),
            spin_axis_ws: spin_axis_ws.unwrap_or_default().into_inner(),
            steering_orientation_chassis_space: steering_orientation_chassis_space.unwrap_or_default().into_inner(),
            spin_velocity: spin_velocity.unwrap_or_default().into_inner(),
            spin_angle: spin_angle.unwrap_or_default().into_inner(),
            skid_energy_density: skid_energy_density.unwrap_or_default().into_inner(),
            side_force: side_force.unwrap_or_default().into_inner(),
            forward_slip_velocity: forward_slip_velocity.unwrap_or_default().into_inner(),
            side_slip_velocity: side_slip_velocity.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpVehicleInstanceWheelInfo<'a>> for Vec<HkpVehicleInstanceWheelInfoVisitor<'a>> {
    fn from(data: &HkpVehicleInstanceWheelInfo<'a>) -> Self {
        vec![
            HkpVehicleInstanceWheelInfoVisitor::ContactPoint(data.contact_point.clone()),
            HkpVehicleInstanceWheelInfoVisitor::ContactFriction(data.contact_friction.into()),
            HkpVehicleInstanceWheelInfoVisitor::ContactBody(data.contact_body.clone().into()),
            HkpVehicleInstanceWheelInfoVisitor::ContactShapeKey(data.contact_shape_key.clone()),
            HkpVehicleInstanceWheelInfoVisitor::HardPointWs(data.hard_point_ws.into()),
            HkpVehicleInstanceWheelInfoVisitor::RayEndPointWs(data.ray_end_point_ws.into()),
            HkpVehicleInstanceWheelInfoVisitor::CurrentSuspensionLength(data.current_suspension_length.into()),
            HkpVehicleInstanceWheelInfoVisitor::SuspensionDirectionWs(data.suspension_direction_ws.into()),
            HkpVehicleInstanceWheelInfoVisitor::SpinAxisChassisSpace(data.spin_axis_chassis_space.into()),
            HkpVehicleInstanceWheelInfoVisitor::SpinAxisWs(data.spin_axis_ws.into()),
            HkpVehicleInstanceWheelInfoVisitor::SteeringOrientationChassisSpace(data.steering_orientation_chassis_space.clone().into()),
            HkpVehicleInstanceWheelInfoVisitor::SpinVelocity(data.spin_velocity.into()),
            HkpVehicleInstanceWheelInfoVisitor::SpinAngle(data.spin_angle.into()),
            HkpVehicleInstanceWheelInfoVisitor::SkidEnergyDensity(data.skid_energy_density.into()),
            HkpVehicleInstanceWheelInfoVisitor::SideForce(data.side_force.into()),
            HkpVehicleInstanceWheelInfoVisitor::ForwardSlipVelocity(data.forward_slip_velocity.into()),
            HkpVehicleInstanceWheelInfoVisitor::SideSlipVelocity(data.side_slip_velocity.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpVehicleInstanceWheelInfo<'de> {
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
enum HkpVehicleInstanceWheelInfoVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "contactPoint")]
    ContactPoint(SingleClass<HkContactPoint>),
    /// Visitor fields
    #[serde(rename = "contactFriction")]
    ContactFriction(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "contactBody", skip_serializing)]
    ContactBody(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "contactShapeKey")]
    ContactShapeKey(CStyleArray<[u32; 8]>),
    /// Visitor fields
    #[serde(rename = "hardPointWs")]
    HardPointWs(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "rayEndPointWs")]
    RayEndPointWs(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "currentSuspensionLength")]
    CurrentSuspensionLength(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "suspensionDirectionWs")]
    SuspensionDirectionWs(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "spinAxisChassisSpace")]
    SpinAxisChassisSpace(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "spinAxisWs")]
    SpinAxisWs(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "steeringOrientationChassisSpace")]
    SteeringOrientationChassisSpace(Primitive<Quaternion<f32>>),
    /// Visitor fields
    #[serde(rename = "spinVelocity")]
    SpinVelocity(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "spinAngle")]
    SpinAngle(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "skidEnergyDensity")]
    SkidEnergyDensity(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "sideForce")]
    SideForce(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "forwardSlipVelocity")]
    ForwardSlipVelocity(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "sideSlipVelocity")]
    SideSlipVelocity(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleInstanceWheelInfoVisitor<'de>, "@name",
    ("contactPoint" => ContactPoint(SingleClass<HkContactPoint>)),
    ("contactFriction" => ContactFriction(Primitive<f32>)),
    ("contactBody" => ContactBody(Primitive<Cow<'de, str>>)),
    ("contactShapeKey" => ContactShapeKey(CStyleArray<[u32; 8]>)),
    ("hardPointWs" => HardPointWs(Primitive<Vector4<f32>>)),
    ("rayEndPointWs" => RayEndPointWs(Primitive<Vector4<f32>>)),
    ("currentSuspensionLength" => CurrentSuspensionLength(Primitive<f32>)),
    ("suspensionDirectionWs" => SuspensionDirectionWs(Primitive<Vector4<f32>>)),
    ("spinAxisChassisSpace" => SpinAxisChassisSpace(Primitive<Vector4<f32>>)),
    ("spinAxisWs" => SpinAxisWs(Primitive<Vector4<f32>>)),
    ("steeringOrientationChassisSpace" => SteeringOrientationChassisSpace(Primitive<Quaternion<f32>>)),
    ("spinVelocity" => SpinVelocity(Primitive<f32>)),
    ("spinAngle" => SpinAngle(Primitive<f32>)),
    ("skidEnergyDensity" => SkidEnergyDensity(Primitive<f32>)),
    ("sideForce" => SideForce(Primitive<f32>)),
    ("forwardSlipVelocity" => ForwardSlipVelocity(Primitive<f32>)),
    ("sideSlipVelocity" => SideSlipVelocity(Primitive<f32>)),
}
