//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpAngConstraintAtom`
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

/// `hkpAngConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 4
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x35bb3cd0`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpAngConstraintAtom {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub _type: AtomType,

    /// # C++ Class Fields Info
    /// -   name:`"firstConstrainedAxis"`
    /// -   type: `hkUint8`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    pub first_constrained_axis: u8,
    /// # C++ Class Fields Info
    /// -   name:`"numConstrainedAxes"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    pub num_constrained_axes: u8,
}

impl Serialize for HkpAngConstraintAtom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpAngConstraintAtomVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpAngConstraintAtom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpAngConstraintAtomVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpAngConstraintAtomVisitor>> for HkpAngConstraintAtom {
    fn from(_values: Vec<HkpAngConstraintAtomVisitor>) -> Self {
            let mut _type = None;
            let mut first_constrained_axis = None;
            let mut num_constrained_axes = None;


        for _value in _values {
            match _value {
                HkpAngConstraintAtomVisitor::Type(m) => _type = Some(m),
                HkpAngConstraintAtomVisitor::FirstConstrainedAxis(m) => first_constrained_axis = Some(m),
                HkpAngConstraintAtomVisitor::NumConstrainedAxes(m) => num_constrained_axes = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),
            first_constrained_axis: first_constrained_axis.unwrap_or_default().into_inner(),
            num_constrained_axes: num_constrained_axes.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpAngConstraintAtom> for Vec<HkpAngConstraintAtomVisitor> {
    fn from(data: &HkpAngConstraintAtom) -> Self {
        vec![
            HkpAngConstraintAtomVisitor::Type(data._type.clone().into()),
            HkpAngConstraintAtomVisitor::FirstConstrainedAxis(data.first_constrained_axis.into()),
            HkpAngConstraintAtomVisitor::NumConstrainedAxes(data.num_constrained_axes.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpAngConstraintAtom {
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
enum HkpAngConstraintAtomVisitor {
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// Visitor fields
    #[serde(rename = "firstConstrainedAxis")]
    FirstConstrainedAxis(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "numConstrainedAxes")]
    NumConstrainedAxes(Primitive<u8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpAngConstraintAtomVisitor, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("firstConstrainedAxis" => FirstConstrainedAxis(Primitive<u8>)),
    ("numConstrainedAxes" => NumConstrainedAxes(Primitive<u8>)),
}
