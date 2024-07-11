//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpLimitedHingeConstraintDataAtoms`
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

/// `hkpLimitedHingeConstraintDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 240
/// -    vtable: false
/// - signature: `0x54c7715b`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpLimitedHingeConstraintDataAtoms<'a> {
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
    /// -   name:`"angMotor"`
    /// -   type: `struct hkpAngMotorConstraintAtom`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    pub ang_motor: SingleClass<HkpAngMotorConstraintAtom<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"angFriction"`
    /// -   type: `struct hkpAngFrictionConstraintAtom`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    pub ang_friction: SingleClass<HkpAngFrictionConstraintAtom>,
    /// # C++ Class Fields Info
    /// -   name:`"angLimit"`
    /// -   type: `struct hkpAngLimitConstraintAtom`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    pub ang_limit: SingleClass<HkpAngLimitConstraintAtom>,
    /// # C++ Class Fields Info
    /// -   name:`"2dAng"`
    /// -   type: `struct hkp2dAngConstraintAtom`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    pub _2_d_ang: SingleClass<Hkp2DAngConstraintAtom>,
    /// # C++ Class Fields Info
    /// -   name:`"ballSocket"`
    /// -   type: `struct hkpBallSocketConstraintAtom`
    /// - offset: 212
    /// -  flags: `FLAGS_NONE`
    pub ball_socket: SingleClass<HkpBallSocketConstraintAtom>,
}

impl Serialize for HkpLimitedHingeConstraintDataAtoms<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpLimitedHingeConstraintDataAtomsVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpLimitedHingeConstraintDataAtoms<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpLimitedHingeConstraintDataAtomsVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpLimitedHingeConstraintDataAtomsVisitor<'a>>> for HkpLimitedHingeConstraintDataAtoms<'a> {
    fn from(_values: Vec<HkpLimitedHingeConstraintDataAtomsVisitor<'a>>) -> Self {
            let mut transforms = None;
            let mut setup_stabilization = None;
            let mut ang_motor = None;
            let mut ang_friction = None;
            let mut ang_limit = None;
            let mut _2_d_ang = None;
            let mut ball_socket = None;


        for _value in _values {
            match _value {
                HkpLimitedHingeConstraintDataAtomsVisitor::Transforms(m) => transforms = Some(m),
                HkpLimitedHingeConstraintDataAtomsVisitor::SetupStabilization(m) => setup_stabilization = Some(m),
                HkpLimitedHingeConstraintDataAtomsVisitor::AngMotor(m) => ang_motor = Some(m),
                HkpLimitedHingeConstraintDataAtomsVisitor::AngFriction(m) => ang_friction = Some(m),
                HkpLimitedHingeConstraintDataAtomsVisitor::AngLimit(m) => ang_limit = Some(m),
                HkpLimitedHingeConstraintDataAtomsVisitor::_2DAng(m) => _2_d_ang = Some(m),
                HkpLimitedHingeConstraintDataAtomsVisitor::BallSocket(m) => ball_socket = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            transforms: transforms.unwrap_or_default(),
            setup_stabilization: setup_stabilization.unwrap_or_default(),
            ang_motor: ang_motor.unwrap_or_default(),
            ang_friction: ang_friction.unwrap_or_default(),
            ang_limit: ang_limit.unwrap_or_default(),
            _2_d_ang: _2_d_ang.unwrap_or_default(),
            ball_socket: ball_socket.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpLimitedHingeConstraintDataAtoms<'a>> for Vec<HkpLimitedHingeConstraintDataAtomsVisitor<'a>> {
    fn from(data: &HkpLimitedHingeConstraintDataAtoms<'a>) -> Self {
        vec![
            HkpLimitedHingeConstraintDataAtomsVisitor::Transforms(data.transforms.clone()),
            HkpLimitedHingeConstraintDataAtomsVisitor::SetupStabilization(data.setup_stabilization.clone()),
            HkpLimitedHingeConstraintDataAtomsVisitor::AngMotor(data.ang_motor.clone()),
            HkpLimitedHingeConstraintDataAtomsVisitor::AngFriction(data.ang_friction.clone()),
            HkpLimitedHingeConstraintDataAtomsVisitor::AngLimit(data.ang_limit.clone()),
            HkpLimitedHingeConstraintDataAtomsVisitor::_2DAng(data._2_d_ang.clone()),
            HkpLimitedHingeConstraintDataAtomsVisitor::BallSocket(data.ball_socket.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpLimitedHingeConstraintDataAtoms<'de> {
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
enum HkpLimitedHingeConstraintDataAtomsVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "transforms")]
    Transforms(SingleClass<HkpSetLocalTransformsConstraintAtom>),
    /// Visitor fields
    #[serde(rename = "setupStabilization")]
    SetupStabilization(SingleClass<HkpSetupStabilizationAtom>),
    /// Visitor fields
    #[serde(rename = "angMotor")]
    AngMotor(SingleClass<HkpAngMotorConstraintAtom<'a>>),
    /// Visitor fields
    #[serde(rename = "angFriction")]
    AngFriction(SingleClass<HkpAngFrictionConstraintAtom>),
    /// Visitor fields
    #[serde(rename = "angLimit")]
    AngLimit(SingleClass<HkpAngLimitConstraintAtom>),
    /// Visitor fields
    #[serde(rename = "2dAng")]
    _2DAng(SingleClass<Hkp2DAngConstraintAtom>),
    /// Visitor fields
    #[serde(rename = "ballSocket")]
    BallSocket(SingleClass<HkpBallSocketConstraintAtom>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpLimitedHingeConstraintDataAtomsVisitor<'de>, "@name",
    ("transforms" => Transforms(SingleClass<HkpSetLocalTransformsConstraintAtom>)),
    ("setupStabilization" => SetupStabilization(SingleClass<HkpSetupStabilizationAtom>)),
    ("angMotor" => AngMotor(SingleClass<HkpAngMotorConstraintAtom<'de>>)),
    ("angFriction" => AngFriction(SingleClass<HkpAngFrictionConstraintAtom>)),
    ("angLimit" => AngLimit(SingleClass<HkpAngLimitConstraintAtom>)),
    ("2dAng" => _2DAng(SingleClass<Hkp2DAngConstraintAtom>)),
    ("ballSocket" => BallSocket(SingleClass<HkpBallSocketConstraintAtom>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum Axis {
    #[serde(rename = "AXIS_AXLE")]
    #[default]
    AxisAxle = 0,
    #[serde(rename = "AXIS_PERP_TO_AXLE_1")]
    AxisPerpToAxle1 = 1,
    #[serde(rename = "AXIS_PERP_TO_AXLE_2")]
    AxisPerpToAxle2 = 2,
}
