//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCharacterRigidBodyCinfo`
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

/// `hkpCharacterRigidBodyCinfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkpCharacterControllerCinfo`/`0xda8c7d7d`
/// - signature: `0x892f441`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpCharacterRigidBodyCinfo<'a> {
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
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub collision_filter_info: u32,
    /// # C++ Class Fields Info
    /// -   name:`"shape"`
    /// -   type: `struct hkpShape*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub shape: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"position"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub position: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"rotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub rotation: Quaternion<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"mass"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub mass: f32,
    /// # C++ Class Fields Info
    /// -   name:`"friction"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub friction: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxLinearVelocity"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub max_linear_velocity: f32,
    /// # C++ Class Fields Info
    /// -   name:`"allowedPenetrationDepth"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub allowed_penetration_depth: f32,
    /// # C++ Class Fields Info
    /// -   name:`"up"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub up: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"maxSlope"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub max_slope: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxForce"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    pub max_force: f32,
    /// # C++ Class Fields Info
    /// -   name:`"unweldingHeightOffsetFactor"`
    /// -   type: `hkReal`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    pub unwelding_height_offset_factor: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxSpeedForSimplexSolver"`
    /// -   type: `hkReal`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    pub max_speed_for_simplex_solver: f32,
    /// # C++ Class Fields Info
    /// -   name:`"supportDistance"`
    /// -   type: `hkReal`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub support_distance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"hardSupportDistance"`
    /// -   type: `hkReal`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    pub hard_support_distance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"vdbColor"`
    /// -   type: `hkInt32`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    pub vdb_color: i32,
}

impl Serialize for HkpCharacterRigidBodyCinfo<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpCharacterRigidBodyCinfoVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpCharacterRigidBodyCinfo<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpCharacterRigidBodyCinfoVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpCharacterRigidBodyCinfoVisitor<'a>>> for HkpCharacterRigidBodyCinfo<'a> {
    fn from(_values: Vec<HkpCharacterRigidBodyCinfoVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut collision_filter_info = None;
            let mut shape = None;
            let mut position = None;
            let mut rotation = None;
            let mut mass = None;
            let mut friction = None;
            let mut max_linear_velocity = None;
            let mut allowed_penetration_depth = None;
            let mut up = None;
            let mut max_slope = None;
            let mut max_force = None;
            let mut unwelding_height_offset_factor = None;
            let mut max_speed_for_simplex_solver = None;
            let mut support_distance = None;
            let mut hard_support_distance = None;
            let mut vdb_color = None;


        for _value in _values {
            match _value {
                HkpCharacterRigidBodyCinfoVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpCharacterRigidBodyCinfoVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpCharacterRigidBodyCinfoVisitor::CollisionFilterInfo(m) => collision_filter_info = Some(m),
                HkpCharacterRigidBodyCinfoVisitor::Shape(m) => shape = Some(m),
                HkpCharacterRigidBodyCinfoVisitor::Position(m) => position = Some(m),
                HkpCharacterRigidBodyCinfoVisitor::Rotation(m) => rotation = Some(m),
                HkpCharacterRigidBodyCinfoVisitor::Mass(m) => mass = Some(m),
                HkpCharacterRigidBodyCinfoVisitor::Friction(m) => friction = Some(m),
                HkpCharacterRigidBodyCinfoVisitor::MaxLinearVelocity(m) => max_linear_velocity = Some(m),
                HkpCharacterRigidBodyCinfoVisitor::AllowedPenetrationDepth(m) => allowed_penetration_depth = Some(m),
                HkpCharacterRigidBodyCinfoVisitor::Up(m) => up = Some(m),
                HkpCharacterRigidBodyCinfoVisitor::MaxSlope(m) => max_slope = Some(m),
                HkpCharacterRigidBodyCinfoVisitor::MaxForce(m) => max_force = Some(m),
                HkpCharacterRigidBodyCinfoVisitor::UnweldingHeightOffsetFactor(m) => unwelding_height_offset_factor = Some(m),
                HkpCharacterRigidBodyCinfoVisitor::MaxSpeedForSimplexSolver(m) => max_speed_for_simplex_solver = Some(m),
                HkpCharacterRigidBodyCinfoVisitor::SupportDistance(m) => support_distance = Some(m),
                HkpCharacterRigidBodyCinfoVisitor::HardSupportDistance(m) => hard_support_distance = Some(m),
                HkpCharacterRigidBodyCinfoVisitor::VdbColor(m) => vdb_color = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            collision_filter_info: collision_filter_info.unwrap_or_default().into_inner(),
            shape: shape.unwrap_or_default().into_inner(),
            position: position.unwrap_or_default().into_inner(),
            rotation: rotation.unwrap_or_default().into_inner(),
            mass: mass.unwrap_or_default().into_inner(),
            friction: friction.unwrap_or_default().into_inner(),
            max_linear_velocity: max_linear_velocity.unwrap_or_default().into_inner(),
            allowed_penetration_depth: allowed_penetration_depth.unwrap_or_default().into_inner(),
            up: up.unwrap_or_default().into_inner(),
            max_slope: max_slope.unwrap_or_default().into_inner(),
            max_force: max_force.unwrap_or_default().into_inner(),
            unwelding_height_offset_factor: unwelding_height_offset_factor.unwrap_or_default().into_inner(),
            max_speed_for_simplex_solver: max_speed_for_simplex_solver.unwrap_or_default().into_inner(),
            support_distance: support_distance.unwrap_or_default().into_inner(),
            hard_support_distance: hard_support_distance.unwrap_or_default().into_inner(),
            vdb_color: vdb_color.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpCharacterRigidBodyCinfo<'a>> for Vec<HkpCharacterRigidBodyCinfoVisitor<'a>> {
    fn from(data: &HkpCharacterRigidBodyCinfo<'a>) -> Self {
        vec![
            HkpCharacterRigidBodyCinfoVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpCharacterRigidBodyCinfoVisitor::ReferenceCount(data.reference_count.into()),
            HkpCharacterRigidBodyCinfoVisitor::CollisionFilterInfo(data.collision_filter_info.into()),
            HkpCharacterRigidBodyCinfoVisitor::Shape(data.shape.clone().into()),
            HkpCharacterRigidBodyCinfoVisitor::Position(data.position.into()),
            HkpCharacterRigidBodyCinfoVisitor::Rotation(data.rotation.clone().into()),
            HkpCharacterRigidBodyCinfoVisitor::Mass(data.mass.into()),
            HkpCharacterRigidBodyCinfoVisitor::Friction(data.friction.into()),
            HkpCharacterRigidBodyCinfoVisitor::MaxLinearVelocity(data.max_linear_velocity.into()),
            HkpCharacterRigidBodyCinfoVisitor::AllowedPenetrationDepth(data.allowed_penetration_depth.into()),
            HkpCharacterRigidBodyCinfoVisitor::Up(data.up.into()),
            HkpCharacterRigidBodyCinfoVisitor::MaxSlope(data.max_slope.into()),
            HkpCharacterRigidBodyCinfoVisitor::MaxForce(data.max_force.into()),
            HkpCharacterRigidBodyCinfoVisitor::UnweldingHeightOffsetFactor(data.unwelding_height_offset_factor.into()),
            HkpCharacterRigidBodyCinfoVisitor::MaxSpeedForSimplexSolver(data.max_speed_for_simplex_solver.into()),
            HkpCharacterRigidBodyCinfoVisitor::SupportDistance(data.support_distance.into()),
            HkpCharacterRigidBodyCinfoVisitor::HardSupportDistance(data.hard_support_distance.into()),
            HkpCharacterRigidBodyCinfoVisitor::VdbColor(data.vdb_color.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpCharacterRigidBodyCinfo<'de> {
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
enum HkpCharacterRigidBodyCinfoVisitor<'a> {
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
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "shape")]
    Shape(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "position")]
    Position(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "rotation")]
    Rotation(Primitive<Quaternion<f32>>),
    /// Visitor fields
    #[serde(rename = "mass")]
    Mass(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "friction")]
    Friction(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxLinearVelocity")]
    MaxLinearVelocity(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "allowedPenetrationDepth")]
    AllowedPenetrationDepth(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "up")]
    Up(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "maxSlope")]
    MaxSlope(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxForce")]
    MaxForce(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "unweldingHeightOffsetFactor")]
    UnweldingHeightOffsetFactor(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxSpeedForSimplexSolver")]
    MaxSpeedForSimplexSolver(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "supportDistance")]
    SupportDistance(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "hardSupportDistance")]
    HardSupportDistance(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "vdbColor")]
    VdbColor(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCharacterRigidBodyCinfoVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("collisionFilterInfo" => CollisionFilterInfo(Primitive<u32>)),
    ("shape" => Shape(Primitive<Cow<'de, str>>)),
    ("position" => Position(Primitive<Vector4<f32>>)),
    ("rotation" => Rotation(Primitive<Quaternion<f32>>)),
    ("mass" => Mass(Primitive<f32>)),
    ("friction" => Friction(Primitive<f32>)),
    ("maxLinearVelocity" => MaxLinearVelocity(Primitive<f32>)),
    ("allowedPenetrationDepth" => AllowedPenetrationDepth(Primitive<f32>)),
    ("up" => Up(Primitive<Vector4<f32>>)),
    ("maxSlope" => MaxSlope(Primitive<f32>)),
    ("maxForce" => MaxForce(Primitive<f32>)),
    ("unweldingHeightOffsetFactor" => UnweldingHeightOffsetFactor(Primitive<f32>)),
    ("maxSpeedForSimplexSolver" => MaxSpeedForSimplexSolver(Primitive<f32>)),
    ("supportDistance" => SupportDistance(Primitive<f32>)),
    ("hardSupportDistance" => HardSupportDistance(Primitive<f32>)),
    ("vdbColor" => VdbColor(Primitive<i32>)),
}
