//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpRagdollLimitsDataAtoms`
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

/// `hkpRagdollLimitsDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 176
/// -    vtable: false
/// - signature: `0x82b894c3`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpRagdollLimitsDataAtoms {
    /// # C++ Class Fields Info
    /// -   name:`"rotations"`
    /// -   type: `struct hkpSetLocalRotationsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub rotations: SingleClass<HkpSetLocalRotationsConstraintAtom>,
    /// # C++ Class Fields Info
    /// -   name:`"twistLimit"`
    /// -   type: `struct hkpTwistLimitConstraintAtom`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    pub twist_limit: SingleClass<HkpTwistLimitConstraintAtom>,
    /// # C++ Class Fields Info
    /// -   name:`"coneLimit"`
    /// -   type: `struct hkpConeLimitConstraintAtom`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE`
    pub cone_limit: SingleClass<HkpConeLimitConstraintAtom>,
    /// # C++ Class Fields Info
    /// -   name:`"planesLimit"`
    /// -   type: `struct hkpConeLimitConstraintAtom`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE`
    pub planes_limit: SingleClass<HkpConeLimitConstraintAtom>,
}

impl Serialize for HkpRagdollLimitsDataAtoms {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpRagdollLimitsDataAtomsVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpRagdollLimitsDataAtoms {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpRagdollLimitsDataAtomsVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpRagdollLimitsDataAtomsVisitor>> for HkpRagdollLimitsDataAtoms {
    fn from(_values: Vec<HkpRagdollLimitsDataAtomsVisitor>) -> Self {
            let mut rotations = None;
            let mut twist_limit = None;
            let mut cone_limit = None;
            let mut planes_limit = None;


        for _value in _values {
            match _value {
                HkpRagdollLimitsDataAtomsVisitor::Rotations(m) => rotations = Some(m),
                HkpRagdollLimitsDataAtomsVisitor::TwistLimit(m) => twist_limit = Some(m),
                HkpRagdollLimitsDataAtomsVisitor::ConeLimit(m) => cone_limit = Some(m),
                HkpRagdollLimitsDataAtomsVisitor::PlanesLimit(m) => planes_limit = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            rotations: rotations.unwrap_or_default(),
            twist_limit: twist_limit.unwrap_or_default(),
            cone_limit: cone_limit.unwrap_or_default(),
            planes_limit: planes_limit.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpRagdollLimitsDataAtoms> for Vec<HkpRagdollLimitsDataAtomsVisitor> {
    fn from(data: &HkpRagdollLimitsDataAtoms) -> Self {
        vec![
            HkpRagdollLimitsDataAtomsVisitor::Rotations(data.rotations.clone()),
            HkpRagdollLimitsDataAtomsVisitor::TwistLimit(data.twist_limit.clone()),
            HkpRagdollLimitsDataAtomsVisitor::ConeLimit(data.cone_limit.clone()),
            HkpRagdollLimitsDataAtomsVisitor::PlanesLimit(data.planes_limit.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpRagdollLimitsDataAtoms {
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
enum HkpRagdollLimitsDataAtomsVisitor {
    /// Visitor fields
    #[serde(rename = "rotations")]
    Rotations(SingleClass<HkpSetLocalRotationsConstraintAtom>),
    /// Visitor fields
    #[serde(rename = "twistLimit")]
    TwistLimit(SingleClass<HkpTwistLimitConstraintAtom>),
    /// Visitor fields
    #[serde(rename = "coneLimit")]
    ConeLimit(SingleClass<HkpConeLimitConstraintAtom>),
    /// Visitor fields
    #[serde(rename = "planesLimit")]
    PlanesLimit(SingleClass<HkpConeLimitConstraintAtom>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpRagdollLimitsDataAtomsVisitor, "@name",
    ("rotations" => Rotations(SingleClass<HkpSetLocalRotationsConstraintAtom>)),
    ("twistLimit" => TwistLimit(SingleClass<HkpTwistLimitConstraintAtom>)),
    ("coneLimit" => ConeLimit(SingleClass<HkpConeLimitConstraintAtom>)),
    ("planesLimit" => PlanesLimit(SingleClass<HkpConeLimitConstraintAtom>)),
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
