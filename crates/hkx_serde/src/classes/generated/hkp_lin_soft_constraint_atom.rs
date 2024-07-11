//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpLinSoftConstraintAtom`
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

/// `hkpLinSoftConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x52b27d69`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpLinSoftConstraintAtom {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub _type: AtomType,

    /// # C++ Class Fields Info
    /// -   name:`"axisIndex"`
    /// -   type: `hkUint8`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    pub axis_index: u8,
    /// # C++ Class Fields Info
    /// -   name:`"tau"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub tau: f32,
    /// # C++ Class Fields Info
    /// -   name:`"damping"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub damping: f32,
}

impl Serialize for HkpLinSoftConstraintAtom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpLinSoftConstraintAtomVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpLinSoftConstraintAtom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpLinSoftConstraintAtomVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpLinSoftConstraintAtomVisitor>> for HkpLinSoftConstraintAtom {
    fn from(_values: Vec<HkpLinSoftConstraintAtomVisitor>) -> Self {
            let mut _type = None;
            let mut axis_index = None;
            let mut tau = None;
            let mut damping = None;


        for _value in _values {
            match _value {
                HkpLinSoftConstraintAtomVisitor::Type(m) => _type = Some(m),
                HkpLinSoftConstraintAtomVisitor::AxisIndex(m) => axis_index = Some(m),
                HkpLinSoftConstraintAtomVisitor::Tau(m) => tau = Some(m),
                HkpLinSoftConstraintAtomVisitor::Damping(m) => damping = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),
            axis_index: axis_index.unwrap_or_default().into_inner(),
            tau: tau.unwrap_or_default().into_inner(),
            damping: damping.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpLinSoftConstraintAtom> for Vec<HkpLinSoftConstraintAtomVisitor> {
    fn from(data: &HkpLinSoftConstraintAtom) -> Self {
        vec![
            HkpLinSoftConstraintAtomVisitor::Type(data._type.clone().into()),
            HkpLinSoftConstraintAtomVisitor::AxisIndex(data.axis_index.into()),
            HkpLinSoftConstraintAtomVisitor::Tau(data.tau.into()),
            HkpLinSoftConstraintAtomVisitor::Damping(data.damping.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpLinSoftConstraintAtom {
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
enum HkpLinSoftConstraintAtomVisitor {
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// Visitor fields
    #[serde(rename = "axisIndex")]
    AxisIndex(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "tau")]
    Tau(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "damping")]
    Damping(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpLinSoftConstraintAtomVisitor, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("axisIndex" => AxisIndex(Primitive<u8>)),
    ("tau" => Tau(Primitive<f32>)),
    ("damping" => Damping(Primitive<f32>)),
}
