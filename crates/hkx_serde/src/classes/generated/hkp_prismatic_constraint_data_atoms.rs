//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPrismaticConstraintDataAtoms`
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

/// `hkpPrismaticConstraintDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 192
/// -    vtable: false
/// - signature: `0x7f516137`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpPrismaticConstraintDataAtoms<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"transforms"`
    /// -   type: `struct hkpSetLocalTransformsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub transforms: SingleClass<HkpSetLocalTransformsConstraintAtom>,
    /// # C++ Class Fields Info
    /// -   name:`"motor"`
    /// -   type: `struct hkpLinMotorConstraintAtom`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    pub motor: SingleClass<HkpLinMotorConstraintAtom<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"friction"`
    /// -   type: `struct hkpLinFrictionConstraintAtom`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    pub friction: SingleClass<HkpLinFrictionConstraintAtom>,
    /// # C++ Class Fields Info
    /// -   name:`"ang"`
    /// -   type: `struct hkpAngConstraintAtom`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    pub ang: SingleClass<HkpAngConstraintAtom>,
    /// # C++ Class Fields Info
    /// -   name:`"lin0"`
    /// -   type: `struct hkpLinConstraintAtom`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    pub lin_0: SingleClass<HkpLinConstraintAtom>,
    /// # C++ Class Fields Info
    /// -   name:`"lin1"`
    /// -   type: `struct hkpLinConstraintAtom`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    pub lin_1: SingleClass<HkpLinConstraintAtom>,
    /// # C++ Class Fields Info
    /// -   name:`"linLimit"`
    /// -   type: `struct hkpLinLimitConstraintAtom`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    pub lin_limit: SingleClass<HkpLinLimitConstraintAtom>,
}

impl Serialize for HkpPrismaticConstraintDataAtoms<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpPrismaticConstraintDataAtomsVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpPrismaticConstraintDataAtoms<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpPrismaticConstraintDataAtomsVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpPrismaticConstraintDataAtomsVisitor<'a>>> for HkpPrismaticConstraintDataAtoms<'a> {
    fn from(_values: Vec<HkpPrismaticConstraintDataAtomsVisitor<'a>>) -> Self {
            let mut transforms = None;
            let mut motor = None;
            let mut friction = None;
            let mut ang = None;
            let mut lin_0 = None;
            let mut lin_1 = None;
            let mut lin_limit = None;


        for _value in _values {
            match _value {
                HkpPrismaticConstraintDataAtomsVisitor::Transforms(m) => transforms = Some(m),
                HkpPrismaticConstraintDataAtomsVisitor::Motor(m) => motor = Some(m),
                HkpPrismaticConstraintDataAtomsVisitor::Friction(m) => friction = Some(m),
                HkpPrismaticConstraintDataAtomsVisitor::Ang(m) => ang = Some(m),
                HkpPrismaticConstraintDataAtomsVisitor::Lin0(m) => lin_0 = Some(m),
                HkpPrismaticConstraintDataAtomsVisitor::Lin1(m) => lin_1 = Some(m),
                HkpPrismaticConstraintDataAtomsVisitor::LinLimit(m) => lin_limit = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            transforms: transforms.unwrap_or_default(),
            motor: motor.unwrap_or_default(),
            friction: friction.unwrap_or_default(),
            ang: ang.unwrap_or_default(),
            lin_0: lin_0.unwrap_or_default(),
            lin_1: lin_1.unwrap_or_default(),
            lin_limit: lin_limit.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpPrismaticConstraintDataAtoms<'a>> for Vec<HkpPrismaticConstraintDataAtomsVisitor<'a>> {
    fn from(data: &HkpPrismaticConstraintDataAtoms<'a>) -> Self {
        vec![
            HkpPrismaticConstraintDataAtomsVisitor::Transforms(data.transforms.clone()),
            HkpPrismaticConstraintDataAtomsVisitor::Motor(data.motor.clone()),
            HkpPrismaticConstraintDataAtomsVisitor::Friction(data.friction.clone()),
            HkpPrismaticConstraintDataAtomsVisitor::Ang(data.ang.clone()),
            HkpPrismaticConstraintDataAtomsVisitor::Lin0(data.lin_0.clone()),
            HkpPrismaticConstraintDataAtomsVisitor::Lin1(data.lin_1.clone()),
            HkpPrismaticConstraintDataAtomsVisitor::LinLimit(data.lin_limit.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpPrismaticConstraintDataAtoms<'de> {
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
enum HkpPrismaticConstraintDataAtomsVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "transforms")]
    Transforms(SingleClass<HkpSetLocalTransformsConstraintAtom>),
    /// Visitor fields
    #[serde(rename = "motor")]
    Motor(SingleClass<HkpLinMotorConstraintAtom<'a>>),
    /// Visitor fields
    #[serde(rename = "friction")]
    Friction(SingleClass<HkpLinFrictionConstraintAtom>),
    /// Visitor fields
    #[serde(rename = "ang")]
    Ang(SingleClass<HkpAngConstraintAtom>),
    /// Visitor fields
    #[serde(rename = "lin0")]
    Lin0(SingleClass<HkpLinConstraintAtom>),
    /// Visitor fields
    #[serde(rename = "lin1")]
    Lin1(SingleClass<HkpLinConstraintAtom>),
    /// Visitor fields
    #[serde(rename = "linLimit")]
    LinLimit(SingleClass<HkpLinLimitConstraintAtom>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPrismaticConstraintDataAtomsVisitor<'de>, "@name",
    ("transforms" => Transforms(SingleClass<HkpSetLocalTransformsConstraintAtom>)),
    ("motor" => Motor(SingleClass<HkpLinMotorConstraintAtom<'de>>)),
    ("friction" => Friction(SingleClass<HkpLinFrictionConstraintAtom>)),
    ("ang" => Ang(SingleClass<HkpAngConstraintAtom>)),
    ("lin0" => Lin0(SingleClass<HkpLinConstraintAtom>)),
    ("lin1" => Lin1(SingleClass<HkpLinConstraintAtom>)),
    ("linLimit" => LinLimit(SingleClass<HkpLinLimitConstraintAtom>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum Axis {
    #[serde(rename = "AXIS_SHAFT")]
    #[default]
    AxisShaft = 0,
    #[serde(rename = "AXIS_PERP_TO_SHAFT")]
    AxisPerpToShaft = 1,
}
