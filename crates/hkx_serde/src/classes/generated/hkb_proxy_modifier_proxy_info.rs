//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbProxyModifierProxyInfo`
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

/// `hkbProxyModifierProxyInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: false
/// - signature: `0x39de637e`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbProxyModifierProxyInfo {
    /// # C++ Class Fields Info
    /// -   name:`"dynamicFriction"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub dynamic_friction: f32,
    /// # C++ Class Fields Info
    /// -   name:`"staticFriction"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub static_friction: f32,
    /// # C++ Class Fields Info
    /// -   name:`"keepContactTolerance"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub keep_contact_tolerance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"up"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub up: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"keepDistance"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub keep_distance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"contactAngleSensitivity"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub contact_angle_sensitivity: f32,
    /// # C++ Class Fields Info
    /// -   name:`"userPlanes"`
    /// -   type: `hkUint32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub user_planes: u32,
    /// # C++ Class Fields Info
    /// -   name:`"maxCharacterSpeedForSolver"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub max_character_speed_for_solver: f32,
    /// # C++ Class Fields Info
    /// -   name:`"characterStrength"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub character_strength: f32,
    /// # C++ Class Fields Info
    /// -   name:`"characterMass"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub character_mass: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxSlope"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub max_slope: f32,
    /// # C++ Class Fields Info
    /// -   name:`"penetrationRecoverySpeed"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub penetration_recovery_speed: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxCastIterations"`
    /// -   type: `hkInt32`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub max_cast_iterations: i32,
    /// # C++ Class Fields Info
    /// -   name:`"refreshManifoldInCheckSupport"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub refresh_manifold_in_check_support: bool,
}

impl Serialize for HkbProxyModifierProxyInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbProxyModifierProxyInfoVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbProxyModifierProxyInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbProxyModifierProxyInfoVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbProxyModifierProxyInfoVisitor>> for HkbProxyModifierProxyInfo {
    fn from(_values: Vec<HkbProxyModifierProxyInfoVisitor>) -> Self {
            let mut dynamic_friction = None;
            let mut static_friction = None;
            let mut keep_contact_tolerance = None;
            let mut up = None;
            let mut keep_distance = None;
            let mut contact_angle_sensitivity = None;
            let mut user_planes = None;
            let mut max_character_speed_for_solver = None;
            let mut character_strength = None;
            let mut character_mass = None;
            let mut max_slope = None;
            let mut penetration_recovery_speed = None;
            let mut max_cast_iterations = None;
            let mut refresh_manifold_in_check_support = None;


        for _value in _values {
            match _value {
                HkbProxyModifierProxyInfoVisitor::DynamicFriction(m) => dynamic_friction = Some(m),
                HkbProxyModifierProxyInfoVisitor::StaticFriction(m) => static_friction = Some(m),
                HkbProxyModifierProxyInfoVisitor::KeepContactTolerance(m) => keep_contact_tolerance = Some(m),
                HkbProxyModifierProxyInfoVisitor::Up(m) => up = Some(m),
                HkbProxyModifierProxyInfoVisitor::KeepDistance(m) => keep_distance = Some(m),
                HkbProxyModifierProxyInfoVisitor::ContactAngleSensitivity(m) => contact_angle_sensitivity = Some(m),
                HkbProxyModifierProxyInfoVisitor::UserPlanes(m) => user_planes = Some(m),
                HkbProxyModifierProxyInfoVisitor::MaxCharacterSpeedForSolver(m) => max_character_speed_for_solver = Some(m),
                HkbProxyModifierProxyInfoVisitor::CharacterStrength(m) => character_strength = Some(m),
                HkbProxyModifierProxyInfoVisitor::CharacterMass(m) => character_mass = Some(m),
                HkbProxyModifierProxyInfoVisitor::MaxSlope(m) => max_slope = Some(m),
                HkbProxyModifierProxyInfoVisitor::PenetrationRecoverySpeed(m) => penetration_recovery_speed = Some(m),
                HkbProxyModifierProxyInfoVisitor::MaxCastIterations(m) => max_cast_iterations = Some(m),
                HkbProxyModifierProxyInfoVisitor::RefreshManifoldInCheckSupport(m) => refresh_manifold_in_check_support = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            dynamic_friction: dynamic_friction.unwrap_or_default().into_inner(),
            static_friction: static_friction.unwrap_or_default().into_inner(),
            keep_contact_tolerance: keep_contact_tolerance.unwrap_or_default().into_inner(),
            up: up.unwrap_or_default().into_inner(),
            keep_distance: keep_distance.unwrap_or_default().into_inner(),
            contact_angle_sensitivity: contact_angle_sensitivity.unwrap_or_default().into_inner(),
            user_planes: user_planes.unwrap_or_default().into_inner(),
            max_character_speed_for_solver: max_character_speed_for_solver.unwrap_or_default().into_inner(),
            character_strength: character_strength.unwrap_or_default().into_inner(),
            character_mass: character_mass.unwrap_or_default().into_inner(),
            max_slope: max_slope.unwrap_or_default().into_inner(),
            penetration_recovery_speed: penetration_recovery_speed.unwrap_or_default().into_inner(),
            max_cast_iterations: max_cast_iterations.unwrap_or_default().into_inner(),
            refresh_manifold_in_check_support: refresh_manifold_in_check_support.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbProxyModifierProxyInfo> for Vec<HkbProxyModifierProxyInfoVisitor> {
    fn from(data: &HkbProxyModifierProxyInfo) -> Self {
        vec![
            HkbProxyModifierProxyInfoVisitor::DynamicFriction(data.dynamic_friction.into()),
            HkbProxyModifierProxyInfoVisitor::StaticFriction(data.static_friction.into()),
            HkbProxyModifierProxyInfoVisitor::KeepContactTolerance(data.keep_contact_tolerance.into()),
            HkbProxyModifierProxyInfoVisitor::Up(data.up.into()),
            HkbProxyModifierProxyInfoVisitor::KeepDistance(data.keep_distance.into()),
            HkbProxyModifierProxyInfoVisitor::ContactAngleSensitivity(data.contact_angle_sensitivity.into()),
            HkbProxyModifierProxyInfoVisitor::UserPlanes(data.user_planes.into()),
            HkbProxyModifierProxyInfoVisitor::MaxCharacterSpeedForSolver(data.max_character_speed_for_solver.into()),
            HkbProxyModifierProxyInfoVisitor::CharacterStrength(data.character_strength.into()),
            HkbProxyModifierProxyInfoVisitor::CharacterMass(data.character_mass.into()),
            HkbProxyModifierProxyInfoVisitor::MaxSlope(data.max_slope.into()),
            HkbProxyModifierProxyInfoVisitor::PenetrationRecoverySpeed(data.penetration_recovery_speed.into()),
            HkbProxyModifierProxyInfoVisitor::MaxCastIterations(data.max_cast_iterations.into()),
            HkbProxyModifierProxyInfoVisitor::RefreshManifoldInCheckSupport(data.refresh_manifold_in_check_support.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbProxyModifierProxyInfo {
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
enum HkbProxyModifierProxyInfoVisitor {
    /// Visitor fields
    #[serde(rename = "dynamicFriction")]
    DynamicFriction(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "staticFriction")]
    StaticFriction(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "keepContactTolerance")]
    KeepContactTolerance(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "up")]
    Up(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "keepDistance")]
    KeepDistance(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "contactAngleSensitivity")]
    ContactAngleSensitivity(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "userPlanes")]
    UserPlanes(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "maxCharacterSpeedForSolver")]
    MaxCharacterSpeedForSolver(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "characterStrength")]
    CharacterStrength(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "characterMass")]
    CharacterMass(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxSlope")]
    MaxSlope(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "penetrationRecoverySpeed")]
    PenetrationRecoverySpeed(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxCastIterations")]
    MaxCastIterations(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "refreshManifoldInCheckSupport")]
    RefreshManifoldInCheckSupport(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbProxyModifierProxyInfoVisitor, "@name",
    ("dynamicFriction" => DynamicFriction(Primitive<f32>)),
    ("staticFriction" => StaticFriction(Primitive<f32>)),
    ("keepContactTolerance" => KeepContactTolerance(Primitive<f32>)),
    ("up" => Up(Primitive<Vector4<f32>>)),
    ("keepDistance" => KeepDistance(Primitive<f32>)),
    ("contactAngleSensitivity" => ContactAngleSensitivity(Primitive<f32>)),
    ("userPlanes" => UserPlanes(Primitive<u32>)),
    ("maxCharacterSpeedForSolver" => MaxCharacterSpeedForSolver(Primitive<f32>)),
    ("characterStrength" => CharacterStrength(Primitive<f32>)),
    ("characterMass" => CharacterMass(Primitive<f32>)),
    ("maxSlope" => MaxSlope(Primitive<f32>)),
    ("penetrationRecoverySpeed" => PenetrationRecoverySpeed(Primitive<f32>)),
    ("maxCastIterations" => MaxCastIterations(Primitive<i32>)),
    ("refreshManifoldInCheckSupport" => RefreshManifoldInCheckSupport(Primitive<bool>)),
}
