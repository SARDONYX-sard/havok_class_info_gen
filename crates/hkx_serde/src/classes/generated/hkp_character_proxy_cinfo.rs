//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCharacterProxyCinfo`
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

/// `hkpCharacterProxyCinfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 144
/// -    vtable: true
/// -    parent: `hkpCharacterControllerCinfo`/`0xda8c7d7d`
/// - signature: `0x586d97b2`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpCharacterProxyCinfo<'a> {
    // C++ Parent class(`hkpCharacterControllerCinfo` => parent: `hkReferencedObject`) has no fields
    //
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub mem_size_and_flags: u16,
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub reference_count: i16,

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// # C++ Class Fields Info
    /// -   name:`"position"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub position: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"velocity"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub velocity: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"dynamicFriction"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub dynamic_friction: f32,
    /// # C++ Class Fields Info
    /// -   name:`"staticFriction"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub static_friction: f32,
    /// # C++ Class Fields Info
    /// -   name:`"keepContactTolerance"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub keep_contact_tolerance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"up"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub up: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"extraUpStaticFriction"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub extra_up_static_friction: f32,
    /// # C++ Class Fields Info
    /// -   name:`"extraDownStaticFriction"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    pub extra_down_static_friction: f32,
    /// # C++ Class Fields Info
    /// -   name:`"shapePhantom"`
    /// -   type: `struct hkpShapePhantom*`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    pub shape_phantom: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"keepDistance"`
    /// -   type: `hkReal`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    pub keep_distance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"contactAngleSensitivity"`
    /// -   type: `hkReal`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub contact_angle_sensitivity: f32,
    /// # C++ Class Fields Info
    /// -   name:`"userPlanes"`
    /// -   type: `hkUint32`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    pub user_planes: u32,
    /// # C++ Class Fields Info
    /// -   name:`"maxCharacterSpeedForSolver"`
    /// -   type: `hkReal`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    pub max_character_speed_for_solver: f32,
    /// # C++ Class Fields Info
    /// -   name:`"characterStrength"`
    /// -   type: `hkReal`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    pub character_strength: f32,
    /// # C++ Class Fields Info
    /// -   name:`"characterMass"`
    /// -   type: `hkReal`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    pub character_mass: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxSlope"`
    /// -   type: `hkReal`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    pub max_slope: f32,
    /// # C++ Class Fields Info
    /// -   name:`"penetrationRecoverySpeed"`
    /// -   type: `hkReal`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    pub penetration_recovery_speed: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxCastIterations"`
    /// -   type: `hkInt32`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    pub max_cast_iterations: i32,
    /// # C++ Class Fields Info
    /// -   name:`"refreshManifoldInCheckSupport"`
    /// -   type: `hkBool`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    pub refresh_manifold_in_check_support: bool,
}

impl Serialize for HkpCharacterProxyCinfo<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpCharacterProxyCinfoVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpCharacterProxyCinfo<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpCharacterProxyCinfoVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpCharacterProxyCinfoVisitor<'a>>> for HkpCharacterProxyCinfo<'a> {
    fn from(_values: Vec<HkpCharacterProxyCinfoVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut position = None;
            let mut velocity = None;
            let mut dynamic_friction = None;
            let mut static_friction = None;
            let mut keep_contact_tolerance = None;
            let mut up = None;
            let mut extra_up_static_friction = None;
            let mut extra_down_static_friction = None;
            let mut shape_phantom = None;
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
                HkpCharacterProxyCinfoVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpCharacterProxyCinfoVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpCharacterProxyCinfoVisitor::Position(m) => position = Some(m),
                HkpCharacterProxyCinfoVisitor::Velocity(m) => velocity = Some(m),
                HkpCharacterProxyCinfoVisitor::DynamicFriction(m) => dynamic_friction = Some(m),
                HkpCharacterProxyCinfoVisitor::StaticFriction(m) => static_friction = Some(m),
                HkpCharacterProxyCinfoVisitor::KeepContactTolerance(m) => keep_contact_tolerance = Some(m),
                HkpCharacterProxyCinfoVisitor::Up(m) => up = Some(m),
                HkpCharacterProxyCinfoVisitor::ExtraUpStaticFriction(m) => extra_up_static_friction = Some(m),
                HkpCharacterProxyCinfoVisitor::ExtraDownStaticFriction(m) => extra_down_static_friction = Some(m),
                HkpCharacterProxyCinfoVisitor::ShapePhantom(m) => shape_phantom = Some(m),
                HkpCharacterProxyCinfoVisitor::KeepDistance(m) => keep_distance = Some(m),
                HkpCharacterProxyCinfoVisitor::ContactAngleSensitivity(m) => contact_angle_sensitivity = Some(m),
                HkpCharacterProxyCinfoVisitor::UserPlanes(m) => user_planes = Some(m),
                HkpCharacterProxyCinfoVisitor::MaxCharacterSpeedForSolver(m) => max_character_speed_for_solver = Some(m),
                HkpCharacterProxyCinfoVisitor::CharacterStrength(m) => character_strength = Some(m),
                HkpCharacterProxyCinfoVisitor::CharacterMass(m) => character_mass = Some(m),
                HkpCharacterProxyCinfoVisitor::MaxSlope(m) => max_slope = Some(m),
                HkpCharacterProxyCinfoVisitor::PenetrationRecoverySpeed(m) => penetration_recovery_speed = Some(m),
                HkpCharacterProxyCinfoVisitor::MaxCastIterations(m) => max_cast_iterations = Some(m),
                HkpCharacterProxyCinfoVisitor::RefreshManifoldInCheckSupport(m) => refresh_manifold_in_check_support = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            position: position.unwrap_or_default().into_inner(),
            velocity: velocity.unwrap_or_default().into_inner(),
            dynamic_friction: dynamic_friction.unwrap_or_default().into_inner(),
            static_friction: static_friction.unwrap_or_default().into_inner(),
            keep_contact_tolerance: keep_contact_tolerance.unwrap_or_default().into_inner(),
            up: up.unwrap_or_default().into_inner(),
            extra_up_static_friction: extra_up_static_friction.unwrap_or_default().into_inner(),
            extra_down_static_friction: extra_down_static_friction.unwrap_or_default().into_inner(),
            shape_phantom: shape_phantom.unwrap_or_default().into_inner(),
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
impl<'a> From<&HkpCharacterProxyCinfo<'a>> for Vec<HkpCharacterProxyCinfoVisitor<'a>> {
    fn from(data: &HkpCharacterProxyCinfo<'a>) -> Self {
        vec![
            HkpCharacterProxyCinfoVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpCharacterProxyCinfoVisitor::ReferenceCount(data.reference_count.into()),
            HkpCharacterProxyCinfoVisitor::Position(data.position.into()),
            HkpCharacterProxyCinfoVisitor::Velocity(data.velocity.into()),
            HkpCharacterProxyCinfoVisitor::DynamicFriction(data.dynamic_friction.into()),
            HkpCharacterProxyCinfoVisitor::StaticFriction(data.static_friction.into()),
            HkpCharacterProxyCinfoVisitor::KeepContactTolerance(data.keep_contact_tolerance.into()),
            HkpCharacterProxyCinfoVisitor::Up(data.up.into()),
            HkpCharacterProxyCinfoVisitor::ExtraUpStaticFriction(data.extra_up_static_friction.into()),
            HkpCharacterProxyCinfoVisitor::ExtraDownStaticFriction(data.extra_down_static_friction.into()),
            HkpCharacterProxyCinfoVisitor::ShapePhantom(data.shape_phantom.clone().into()),
            HkpCharacterProxyCinfoVisitor::KeepDistance(data.keep_distance.into()),
            HkpCharacterProxyCinfoVisitor::ContactAngleSensitivity(data.contact_angle_sensitivity.into()),
            HkpCharacterProxyCinfoVisitor::UserPlanes(data.user_planes.into()),
            HkpCharacterProxyCinfoVisitor::MaxCharacterSpeedForSolver(data.max_character_speed_for_solver.into()),
            HkpCharacterProxyCinfoVisitor::CharacterStrength(data.character_strength.into()),
            HkpCharacterProxyCinfoVisitor::CharacterMass(data.character_mass.into()),
            HkpCharacterProxyCinfoVisitor::MaxSlope(data.max_slope.into()),
            HkpCharacterProxyCinfoVisitor::PenetrationRecoverySpeed(data.penetration_recovery_speed.into()),
            HkpCharacterProxyCinfoVisitor::MaxCastIterations(data.max_cast_iterations.into()),
            HkpCharacterProxyCinfoVisitor::RefreshManifoldInCheckSupport(data.refresh_manifold_in_check_support.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpCharacterProxyCinfo<'de> {
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
enum HkpCharacterProxyCinfoVisitor<'a> {
    // C++ Parent class(`hkpCharacterControllerCinfo` => parent: `hkReferencedObject`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "position")]
    Position(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "velocity")]
    Velocity(Primitive<Vector4<f32>>),
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
    #[serde(rename = "extraUpStaticFriction")]
    ExtraUpStaticFriction(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "extraDownStaticFriction")]
    ExtraDownStaticFriction(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "shapePhantom")]
    ShapePhantom(Primitive<Cow<'a, str>>),
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
    HkpCharacterProxyCinfoVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("position" => Position(Primitive<Vector4<f32>>)),
    ("velocity" => Velocity(Primitive<Vector4<f32>>)),
    ("dynamicFriction" => DynamicFriction(Primitive<f32>)),
    ("staticFriction" => StaticFriction(Primitive<f32>)),
    ("keepContactTolerance" => KeepContactTolerance(Primitive<f32>)),
    ("up" => Up(Primitive<Vector4<f32>>)),
    ("extraUpStaticFriction" => ExtraUpStaticFriction(Primitive<f32>)),
    ("extraDownStaticFriction" => ExtraDownStaticFriction(Primitive<f32>)),
    ("shapePhantom" => ShapePhantom(Primitive<Cow<'de, str>>)),
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
