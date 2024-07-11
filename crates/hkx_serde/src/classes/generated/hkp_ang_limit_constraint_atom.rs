//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpAngLimitConstraintAtom`
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

/// `hkpAngLimitConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x9be0d9d`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpAngLimitConstraintAtom {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub _type: AtomType,

    /// # C++ Class Fields Info
    /// -   name:`"isEnabled"`
    /// -   type: `hkUint8`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    pub is_enabled: u8,
    /// # C++ Class Fields Info
    /// -   name:`"limitAxis"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    pub limit_axis: u8,
    /// # C++ Class Fields Info
    /// -   name:`"minAngle"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub min_angle: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxAngle"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub max_angle: f32,
    /// # C++ Class Fields Info
    /// -   name:`"angularLimitsTauFactor"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub angular_limits_tau_factor: f32,
}

impl Serialize for HkpAngLimitConstraintAtom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpAngLimitConstraintAtomVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpAngLimitConstraintAtom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpAngLimitConstraintAtomVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpAngLimitConstraintAtomVisitor>> for HkpAngLimitConstraintAtom {
    fn from(_values: Vec<HkpAngLimitConstraintAtomVisitor>) -> Self {
            let mut _type = None;
            let mut is_enabled = None;
            let mut limit_axis = None;
            let mut min_angle = None;
            let mut max_angle = None;
            let mut angular_limits_tau_factor = None;


        for _value in _values {
            match _value {
                HkpAngLimitConstraintAtomVisitor::Type(m) => _type = Some(m),
                HkpAngLimitConstraintAtomVisitor::IsEnabled(m) => is_enabled = Some(m),
                HkpAngLimitConstraintAtomVisitor::LimitAxis(m) => limit_axis = Some(m),
                HkpAngLimitConstraintAtomVisitor::MinAngle(m) => min_angle = Some(m),
                HkpAngLimitConstraintAtomVisitor::MaxAngle(m) => max_angle = Some(m),
                HkpAngLimitConstraintAtomVisitor::AngularLimitsTauFactor(m) => angular_limits_tau_factor = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),
            is_enabled: is_enabled.unwrap_or_default().into_inner(),
            limit_axis: limit_axis.unwrap_or_default().into_inner(),
            min_angle: min_angle.unwrap_or_default().into_inner(),
            max_angle: max_angle.unwrap_or_default().into_inner(),
            angular_limits_tau_factor: angular_limits_tau_factor.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpAngLimitConstraintAtom> for Vec<HkpAngLimitConstraintAtomVisitor> {
    fn from(data: &HkpAngLimitConstraintAtom) -> Self {
        vec![
            HkpAngLimitConstraintAtomVisitor::Type(data._type.clone().into()),
            HkpAngLimitConstraintAtomVisitor::IsEnabled(data.is_enabled.into()),
            HkpAngLimitConstraintAtomVisitor::LimitAxis(data.limit_axis.into()),
            HkpAngLimitConstraintAtomVisitor::MinAngle(data.min_angle.into()),
            HkpAngLimitConstraintAtomVisitor::MaxAngle(data.max_angle.into()),
            HkpAngLimitConstraintAtomVisitor::AngularLimitsTauFactor(data.angular_limits_tau_factor.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpAngLimitConstraintAtom {
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
enum HkpAngLimitConstraintAtomVisitor {
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// Visitor fields
    #[serde(rename = "isEnabled")]
    IsEnabled(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "limitAxis")]
    LimitAxis(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "minAngle")]
    MinAngle(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxAngle")]
    MaxAngle(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "angularLimitsTauFactor")]
    AngularLimitsTauFactor(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpAngLimitConstraintAtomVisitor, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("isEnabled" => IsEnabled(Primitive<u8>)),
    ("limitAxis" => LimitAxis(Primitive<u8>)),
    ("minAngle" => MinAngle(Primitive<f32>)),
    ("maxAngle" => MaxAngle(Primitive<f32>)),
    ("angularLimitsTauFactor" => AngularLimitsTauFactor(Primitive<f32>)),
}
