//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleFrictionDescriptionAxisDescription`
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

/// `hkpVehicleFrictionDescriptionAxisDescription`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 100
/// -    vtable: false
/// - signature: `0x59ce153f`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpVehicleFrictionDescriptionAxisDescription {
    /// # C++ Class Fields Info
    /// -   name:`"frictionCircleYtab"`
    /// -   type: `hkReal[16]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub friction_circle_ytab: CStyleArray<[f32; 16]>,
    /// # C++ Class Fields Info
    /// -   name:`"xStep"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub x_step: f32,
    /// # C++ Class Fields Info
    /// -   name:`"xStart"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub x_start: f32,
    /// # C++ Class Fields Info
    /// -   name:`"wheelSurfaceInertia"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    pub wheel_surface_inertia: f32,
    /// # C++ Class Fields Info
    /// -   name:`"wheelSurfaceInertiaInv"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    pub wheel_surface_inertia_inv: f32,
    /// # C++ Class Fields Info
    /// -   name:`"wheelChassisMassRatio"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub wheel_chassis_mass_ratio: f32,
    /// # C++ Class Fields Info
    /// -   name:`"wheelRadius"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    pub wheel_radius: f32,
    /// # C++ Class Fields Info
    /// -   name:`"wheelRadiusInv"`
    /// -   type: `hkReal`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    pub wheel_radius_inv: f32,
    /// # C++ Class Fields Info
    /// -   name:`"wheelDownForceFactor"`
    /// -   type: `hkReal`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    pub wheel_down_force_factor: f32,
    /// # C++ Class Fields Info
    /// -   name:`"wheelDownForceSumFactor"`
    /// -   type: `hkReal`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub wheel_down_force_sum_factor: f32,
}

impl Serialize for HkpVehicleFrictionDescriptionAxisDescription {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpVehicleFrictionDescriptionAxisDescriptionVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpVehicleFrictionDescriptionAxisDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpVehicleFrictionDescriptionAxisDescriptionVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpVehicleFrictionDescriptionAxisDescriptionVisitor>> for HkpVehicleFrictionDescriptionAxisDescription {
    fn from(_values: Vec<HkpVehicleFrictionDescriptionAxisDescriptionVisitor>) -> Self {
            let mut friction_circle_ytab = None;
            let mut x_step = None;
            let mut x_start = None;
            let mut wheel_surface_inertia = None;
            let mut wheel_surface_inertia_inv = None;
            let mut wheel_chassis_mass_ratio = None;
            let mut wheel_radius = None;
            let mut wheel_radius_inv = None;
            let mut wheel_down_force_factor = None;
            let mut wheel_down_force_sum_factor = None;


        for _value in _values {
            match _value {
                HkpVehicleFrictionDescriptionAxisDescriptionVisitor::FrictionCircleYtab(m) => friction_circle_ytab = Some(m),
                HkpVehicleFrictionDescriptionAxisDescriptionVisitor::XStep(m) => x_step = Some(m),
                HkpVehicleFrictionDescriptionAxisDescriptionVisitor::XStart(m) => x_start = Some(m),
                HkpVehicleFrictionDescriptionAxisDescriptionVisitor::WheelSurfaceInertia(m) => wheel_surface_inertia = Some(m),
                HkpVehicleFrictionDescriptionAxisDescriptionVisitor::WheelSurfaceInertiaInv(m) => wheel_surface_inertia_inv = Some(m),
                HkpVehicleFrictionDescriptionAxisDescriptionVisitor::WheelChassisMassRatio(m) => wheel_chassis_mass_ratio = Some(m),
                HkpVehicleFrictionDescriptionAxisDescriptionVisitor::WheelRadius(m) => wheel_radius = Some(m),
                HkpVehicleFrictionDescriptionAxisDescriptionVisitor::WheelRadiusInv(m) => wheel_radius_inv = Some(m),
                HkpVehicleFrictionDescriptionAxisDescriptionVisitor::WheelDownForceFactor(m) => wheel_down_force_factor = Some(m),
                HkpVehicleFrictionDescriptionAxisDescriptionVisitor::WheelDownForceSumFactor(m) => wheel_down_force_sum_factor = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            friction_circle_ytab: friction_circle_ytab.unwrap_or_default(),
            x_step: x_step.unwrap_or_default().into_inner(),
            x_start: x_start.unwrap_or_default().into_inner(),
            wheel_surface_inertia: wheel_surface_inertia.unwrap_or_default().into_inner(),
            wheel_surface_inertia_inv: wheel_surface_inertia_inv.unwrap_or_default().into_inner(),
            wheel_chassis_mass_ratio: wheel_chassis_mass_ratio.unwrap_or_default().into_inner(),
            wheel_radius: wheel_radius.unwrap_or_default().into_inner(),
            wheel_radius_inv: wheel_radius_inv.unwrap_or_default().into_inner(),
            wheel_down_force_factor: wheel_down_force_factor.unwrap_or_default().into_inner(),
            wheel_down_force_sum_factor: wheel_down_force_sum_factor.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpVehicleFrictionDescriptionAxisDescription> for Vec<HkpVehicleFrictionDescriptionAxisDescriptionVisitor> {
    fn from(data: &HkpVehicleFrictionDescriptionAxisDescription) -> Self {
        vec![
            HkpVehicleFrictionDescriptionAxisDescriptionVisitor::FrictionCircleYtab(data.friction_circle_ytab.clone()),
            HkpVehicleFrictionDescriptionAxisDescriptionVisitor::XStep(data.x_step.into()),
            HkpVehicleFrictionDescriptionAxisDescriptionVisitor::XStart(data.x_start.into()),
            HkpVehicleFrictionDescriptionAxisDescriptionVisitor::WheelSurfaceInertia(data.wheel_surface_inertia.into()),
            HkpVehicleFrictionDescriptionAxisDescriptionVisitor::WheelSurfaceInertiaInv(data.wheel_surface_inertia_inv.into()),
            HkpVehicleFrictionDescriptionAxisDescriptionVisitor::WheelChassisMassRatio(data.wheel_chassis_mass_ratio.into()),
            HkpVehicleFrictionDescriptionAxisDescriptionVisitor::WheelRadius(data.wheel_radius.into()),
            HkpVehicleFrictionDescriptionAxisDescriptionVisitor::WheelRadiusInv(data.wheel_radius_inv.into()),
            HkpVehicleFrictionDescriptionAxisDescriptionVisitor::WheelDownForceFactor(data.wheel_down_force_factor.into()),
            HkpVehicleFrictionDescriptionAxisDescriptionVisitor::WheelDownForceSumFactor(data.wheel_down_force_sum_factor.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpVehicleFrictionDescriptionAxisDescription {
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
enum HkpVehicleFrictionDescriptionAxisDescriptionVisitor {
    /// Visitor fields
    #[serde(rename = "frictionCircleYtab")]
    FrictionCircleYtab(CStyleArray<[f32; 16]>),
    /// Visitor fields
    #[serde(rename = "xStep")]
    XStep(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "xStart")]
    XStart(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "wheelSurfaceInertia")]
    WheelSurfaceInertia(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "wheelSurfaceInertiaInv")]
    WheelSurfaceInertiaInv(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "wheelChassisMassRatio")]
    WheelChassisMassRatio(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "wheelRadius")]
    WheelRadius(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "wheelRadiusInv")]
    WheelRadiusInv(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "wheelDownForceFactor")]
    WheelDownForceFactor(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "wheelDownForceSumFactor")]
    WheelDownForceSumFactor(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleFrictionDescriptionAxisDescriptionVisitor, "@name",
    ("frictionCircleYtab" => FrictionCircleYtab(CStyleArray<[f32; 16]>)),
    ("xStep" => XStep(Primitive<f32>)),
    ("xStart" => XStart(Primitive<f32>)),
    ("wheelSurfaceInertia" => WheelSurfaceInertia(Primitive<f32>)),
    ("wheelSurfaceInertiaInv" => WheelSurfaceInertiaInv(Primitive<f32>)),
    ("wheelChassisMassRatio" => WheelChassisMassRatio(Primitive<f32>)),
    ("wheelRadius" => WheelRadius(Primitive<f32>)),
    ("wheelRadiusInv" => WheelRadiusInv(Primitive<f32>)),
    ("wheelDownForceFactor" => WheelDownForceFactor(Primitive<f32>)),
    ("wheelDownForceSumFactor" => WheelDownForceSumFactor(Primitive<f32>)),
}
