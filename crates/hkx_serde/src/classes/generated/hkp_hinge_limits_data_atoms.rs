//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpHingeLimitsDataAtoms`
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

/// `hkpHingeLimitsDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 144
/// -    vtable: false
/// - signature: `0x555876ff`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpHingeLimitsDataAtoms {
    /// # C++ Class Fields Info
    /// -   name:`"rotations"`
    /// -   type: `struct hkpSetLocalRotationsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub rotations: SingleClass<HkpSetLocalRotationsConstraintAtom>,
    /// # C++ Class Fields Info
    /// -   name:`"angLimit"`
    /// -   type: `struct hkpAngLimitConstraintAtom`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    pub ang_limit: SingleClass<HkpAngLimitConstraintAtom>,
    /// # C++ Class Fields Info
    /// -   name:`"2dAng"`
    /// -   type: `struct hkp2dAngConstraintAtom`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    pub _2_d_ang: SingleClass<Hkp2DAngConstraintAtom>,
}

impl Serialize for HkpHingeLimitsDataAtoms {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpHingeLimitsDataAtomsVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpHingeLimitsDataAtoms {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpHingeLimitsDataAtomsVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpHingeLimitsDataAtomsVisitor>> for HkpHingeLimitsDataAtoms {
    fn from(_values: Vec<HkpHingeLimitsDataAtomsVisitor>) -> Self {
            let mut rotations = None;
            let mut ang_limit = None;
            let mut _2_d_ang = None;


        for _value in _values {
            match _value {
                HkpHingeLimitsDataAtomsVisitor::Rotations(m) => rotations = Some(m),
                HkpHingeLimitsDataAtomsVisitor::AngLimit(m) => ang_limit = Some(m),
                HkpHingeLimitsDataAtomsVisitor::_2DAng(m) => _2_d_ang = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            rotations: rotations.unwrap_or_default(),
            ang_limit: ang_limit.unwrap_or_default(),
            _2_d_ang: _2_d_ang.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpHingeLimitsDataAtoms> for Vec<HkpHingeLimitsDataAtomsVisitor> {
    fn from(data: &HkpHingeLimitsDataAtoms) -> Self {
        vec![
            HkpHingeLimitsDataAtomsVisitor::Rotations(data.rotations.clone()),
            HkpHingeLimitsDataAtomsVisitor::AngLimit(data.ang_limit.clone()),
            HkpHingeLimitsDataAtomsVisitor::_2DAng(data._2_d_ang.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpHingeLimitsDataAtoms {
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
enum HkpHingeLimitsDataAtomsVisitor {
    /// Visitor fields
    #[serde(rename = "rotations")]
    Rotations(SingleClass<HkpSetLocalRotationsConstraintAtom>),
    /// Visitor fields
    #[serde(rename = "angLimit")]
    AngLimit(SingleClass<HkpAngLimitConstraintAtom>),
    /// Visitor fields
    #[serde(rename = "2dAng")]
    _2DAng(SingleClass<Hkp2DAngConstraintAtom>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpHingeLimitsDataAtomsVisitor, "@name",
    ("rotations" => Rotations(SingleClass<HkpSetLocalRotationsConstraintAtom>)),
    ("angLimit" => AngLimit(SingleClass<HkpAngLimitConstraintAtom>)),
    ("2dAng" => _2DAng(SingleClass<Hkp2DAngConstraintAtom>)),
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
