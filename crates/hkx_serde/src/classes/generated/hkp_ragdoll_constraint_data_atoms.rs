//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpRagdollConstraintDataAtoms`
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

/// `hkpRagdollConstraintDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 336
/// -    vtable: false
/// - signature: `0xeed76b00`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpRagdollConstraintDataAtoms<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"transforms"`
    /// -   type: `struct hkpSetLocalTransformsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub transforms: SingleClass<HkpSetLocalTransformsConstraintAtom>,
    /// # C++ Class Fields Info
    /// -   name:`"setupStabilization"`
    /// -   type: `struct hkpSetupStabilizationAtom`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    pub setup_stabilization: SingleClass<HkpSetupStabilizationAtom>,
    /// # C++ Class Fields Info
    /// -   name:`"ragdollMotors"`
    /// -   type: `struct hkpRagdollMotorConstraintAtom`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    pub ragdoll_motors: SingleClass<HkpRagdollMotorConstraintAtom<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"angFriction"`
    /// -   type: `struct hkpAngFrictionConstraintAtom`
    /// - offset: 240
    /// -  flags: `FLAGS_NONE`
    pub ang_friction: SingleClass<HkpAngFrictionConstraintAtom>,
    /// # C++ Class Fields Info
    /// -   name:`"twistLimit"`
    /// -   type: `struct hkpTwistLimitConstraintAtom`
    /// - offset: 252
    /// -  flags: `FLAGS_NONE`
    pub twist_limit: SingleClass<HkpTwistLimitConstraintAtom>,
    /// # C++ Class Fields Info
    /// -   name:`"coneLimit"`
    /// -   type: `struct hkpConeLimitConstraintAtom`
    /// - offset: 272
    /// -  flags: `FLAGS_NONE`
    pub cone_limit: SingleClass<HkpConeLimitConstraintAtom>,
    /// # C++ Class Fields Info
    /// -   name:`"planesLimit"`
    /// -   type: `struct hkpConeLimitConstraintAtom`
    /// - offset: 292
    /// -  flags: `FLAGS_NONE`
    pub planes_limit: SingleClass<HkpConeLimitConstraintAtom>,
    /// # C++ Class Fields Info
    /// -   name:`"ballSocket"`
    /// -   type: `struct hkpBallSocketConstraintAtom`
    /// - offset: 312
    /// -  flags: `FLAGS_NONE`
    pub ball_socket: SingleClass<HkpBallSocketConstraintAtom>,
}

impl Serialize for HkpRagdollConstraintDataAtoms<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpRagdollConstraintDataAtomsVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpRagdollConstraintDataAtoms<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpRagdollConstraintDataAtomsVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpRagdollConstraintDataAtomsVisitor<'a>>> for HkpRagdollConstraintDataAtoms<'a> {
    fn from(_values: Vec<HkpRagdollConstraintDataAtomsVisitor<'a>>) -> Self {
            let mut transforms = None;
            let mut setup_stabilization = None;
            let mut ragdoll_motors = None;
            let mut ang_friction = None;
            let mut twist_limit = None;
            let mut cone_limit = None;
            let mut planes_limit = None;
            let mut ball_socket = None;


        for _value in _values {
            match _value {
                HkpRagdollConstraintDataAtomsVisitor::Transforms(m) => transforms = Some(m),
                HkpRagdollConstraintDataAtomsVisitor::SetupStabilization(m) => setup_stabilization = Some(m),
                HkpRagdollConstraintDataAtomsVisitor::RagdollMotors(m) => ragdoll_motors = Some(m),
                HkpRagdollConstraintDataAtomsVisitor::AngFriction(m) => ang_friction = Some(m),
                HkpRagdollConstraintDataAtomsVisitor::TwistLimit(m) => twist_limit = Some(m),
                HkpRagdollConstraintDataAtomsVisitor::ConeLimit(m) => cone_limit = Some(m),
                HkpRagdollConstraintDataAtomsVisitor::PlanesLimit(m) => planes_limit = Some(m),
                HkpRagdollConstraintDataAtomsVisitor::BallSocket(m) => ball_socket = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            transforms: transforms.unwrap_or_default(),
            setup_stabilization: setup_stabilization.unwrap_or_default(),
            ragdoll_motors: ragdoll_motors.unwrap_or_default(),
            ang_friction: ang_friction.unwrap_or_default(),
            twist_limit: twist_limit.unwrap_or_default(),
            cone_limit: cone_limit.unwrap_or_default(),
            planes_limit: planes_limit.unwrap_or_default(),
            ball_socket: ball_socket.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpRagdollConstraintDataAtoms<'a>> for Vec<HkpRagdollConstraintDataAtomsVisitor<'a>> {
    fn from(data: &HkpRagdollConstraintDataAtoms<'a>) -> Self {
        vec![
            HkpRagdollConstraintDataAtomsVisitor::Transforms(data.transforms.clone()),
            HkpRagdollConstraintDataAtomsVisitor::SetupStabilization(data.setup_stabilization.clone()),
            HkpRagdollConstraintDataAtomsVisitor::RagdollMotors(data.ragdoll_motors.clone()),
            HkpRagdollConstraintDataAtomsVisitor::AngFriction(data.ang_friction.clone()),
            HkpRagdollConstraintDataAtomsVisitor::TwistLimit(data.twist_limit.clone()),
            HkpRagdollConstraintDataAtomsVisitor::ConeLimit(data.cone_limit.clone()),
            HkpRagdollConstraintDataAtomsVisitor::PlanesLimit(data.planes_limit.clone()),
            HkpRagdollConstraintDataAtomsVisitor::BallSocket(data.ball_socket.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpRagdollConstraintDataAtoms<'de> {
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
enum HkpRagdollConstraintDataAtomsVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "transforms")]
    Transforms(SingleClass<HkpSetLocalTransformsConstraintAtom>),
    /// Visitor fields
    #[serde(rename = "setupStabilization")]
    SetupStabilization(SingleClass<HkpSetupStabilizationAtom>),
    /// Visitor fields
    #[serde(rename = "ragdollMotors")]
    RagdollMotors(SingleClass<HkpRagdollMotorConstraintAtom<'a>>),
    /// Visitor fields
    #[serde(rename = "angFriction")]
    AngFriction(SingleClass<HkpAngFrictionConstraintAtom>),
    /// Visitor fields
    #[serde(rename = "twistLimit")]
    TwistLimit(SingleClass<HkpTwistLimitConstraintAtom>),
    /// Visitor fields
    #[serde(rename = "coneLimit")]
    ConeLimit(SingleClass<HkpConeLimitConstraintAtom>),
    /// Visitor fields
    #[serde(rename = "planesLimit")]
    PlanesLimit(SingleClass<HkpConeLimitConstraintAtom>),
    /// Visitor fields
    #[serde(rename = "ballSocket")]
    BallSocket(SingleClass<HkpBallSocketConstraintAtom>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpRagdollConstraintDataAtomsVisitor<'de>, "@name",
    ("transforms" => Transforms(SingleClass<HkpSetLocalTransformsConstraintAtom>)),
    ("setupStabilization" => SetupStabilization(SingleClass<HkpSetupStabilizationAtom>)),
    ("ragdollMotors" => RagdollMotors(SingleClass<HkpRagdollMotorConstraintAtom<'de>>)),
    ("angFriction" => AngFriction(SingleClass<HkpAngFrictionConstraintAtom>)),
    ("twistLimit" => TwistLimit(SingleClass<HkpTwistLimitConstraintAtom>)),
    ("coneLimit" => ConeLimit(SingleClass<HkpConeLimitConstraintAtom>)),
    ("planesLimit" => PlanesLimit(SingleClass<HkpConeLimitConstraintAtom>)),
    ("ballSocket" => BallSocket(SingleClass<HkpBallSocketConstraintAtom>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum Axis {
    #[serde(rename = "AXIS_TWIST")]
    #[default]
    AxisTwist = 0,
    #[serde(rename = "AXIS_PLANES")]
    AxisPlanes = 1,
    #[serde(rename = "AXIS_CROSS_PRODUCT")]
    AxisCrossProduct = 2,
}
