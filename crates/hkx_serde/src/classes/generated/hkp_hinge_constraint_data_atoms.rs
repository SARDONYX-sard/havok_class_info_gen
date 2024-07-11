//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpHingeConstraintDataAtoms`
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

/// `hkpHingeConstraintDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 192
/// -    vtable: false
/// - signature: `0x6958371c`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpHingeConstraintDataAtoms {
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
    /// -   name:`"2dAng"`
    /// -   type: `struct hkp2dAngConstraintAtom`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    pub _2_d_ang: SingleClass<Hkp2DAngConstraintAtom>,
    /// # C++ Class Fields Info
    /// -   name:`"ballSocket"`
    /// -   type: `struct hkpBallSocketConstraintAtom`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    pub ball_socket: SingleClass<HkpBallSocketConstraintAtom>,
}

impl Serialize for HkpHingeConstraintDataAtoms {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpHingeConstraintDataAtomsVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpHingeConstraintDataAtoms {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpHingeConstraintDataAtomsVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpHingeConstraintDataAtomsVisitor>> for HkpHingeConstraintDataAtoms {
    fn from(_values: Vec<HkpHingeConstraintDataAtomsVisitor>) -> Self {
            let mut transforms = None;
            let mut setup_stabilization = None;
            let mut _2_d_ang = None;
            let mut ball_socket = None;


        for _value in _values {
            match _value {
                HkpHingeConstraintDataAtomsVisitor::Transforms(m) => transforms = Some(m),
                HkpHingeConstraintDataAtomsVisitor::SetupStabilization(m) => setup_stabilization = Some(m),
                HkpHingeConstraintDataAtomsVisitor::_2DAng(m) => _2_d_ang = Some(m),
                HkpHingeConstraintDataAtomsVisitor::BallSocket(m) => ball_socket = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            transforms: transforms.unwrap_or_default(),
            setup_stabilization: setup_stabilization.unwrap_or_default(),
            _2_d_ang: _2_d_ang.unwrap_or_default(),
            ball_socket: ball_socket.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpHingeConstraintDataAtoms> for Vec<HkpHingeConstraintDataAtomsVisitor> {
    fn from(data: &HkpHingeConstraintDataAtoms) -> Self {
        vec![
            HkpHingeConstraintDataAtomsVisitor::Transforms(data.transforms.clone()),
            HkpHingeConstraintDataAtomsVisitor::SetupStabilization(data.setup_stabilization.clone()),
            HkpHingeConstraintDataAtomsVisitor::_2DAng(data._2_d_ang.clone()),
            HkpHingeConstraintDataAtomsVisitor::BallSocket(data.ball_socket.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpHingeConstraintDataAtoms {
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
enum HkpHingeConstraintDataAtomsVisitor {
    /// Visitor fields
    #[serde(rename = "transforms")]
    Transforms(SingleClass<HkpSetLocalTransformsConstraintAtom>),
    /// Visitor fields
    #[serde(rename = "setupStabilization")]
    SetupStabilization(SingleClass<HkpSetupStabilizationAtom>),
    /// Visitor fields
    #[serde(rename = "2dAng")]
    _2DAng(SingleClass<Hkp2DAngConstraintAtom>),
    /// Visitor fields
    #[serde(rename = "ballSocket")]
    BallSocket(SingleClass<HkpBallSocketConstraintAtom>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpHingeConstraintDataAtomsVisitor, "@name",
    ("transforms" => Transforms(SingleClass<HkpSetLocalTransformsConstraintAtom>)),
    ("setupStabilization" => SetupStabilization(SingleClass<HkpSetupStabilizationAtom>)),
    ("2dAng" => _2DAng(SingleClass<Hkp2DAngConstraintAtom>)),
    ("ballSocket" => BallSocket(SingleClass<HkpBallSocketConstraintAtom>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum Axis {
    #[serde(rename = "AXIS_AXLE")]
    #[default]
    AxisAxle = 0,
}
