//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpRotationalConstraintDataAtoms`
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

/// `hkpRotationalConstraintDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 128
/// -    vtable: false
/// - signature: `0xa0c64586`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpRotationalConstraintDataAtoms {
    /// # C++ Class Fields Info
    /// -   name:`"rotations"`
    /// -   type: `struct hkpSetLocalRotationsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub rotations: SingleClass<HkpSetLocalRotationsConstraintAtom>,
    /// # C++ Class Fields Info
    /// -   name:`"ang"`
    /// -   type: `struct hkpAngConstraintAtom`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    pub ang: SingleClass<HkpAngConstraintAtom>,
}

impl Serialize for HkpRotationalConstraintDataAtoms {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpRotationalConstraintDataAtomsVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpRotationalConstraintDataAtoms {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpRotationalConstraintDataAtomsVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpRotationalConstraintDataAtomsVisitor>> for HkpRotationalConstraintDataAtoms {
    fn from(_values: Vec<HkpRotationalConstraintDataAtomsVisitor>) -> Self {
            let mut rotations = None;
            let mut ang = None;


        for _value in _values {
            match _value {
                HkpRotationalConstraintDataAtomsVisitor::Rotations(m) => rotations = Some(m),
                HkpRotationalConstraintDataAtomsVisitor::Ang(m) => ang = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            rotations: rotations.unwrap_or_default(),
            ang: ang.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpRotationalConstraintDataAtoms> for Vec<HkpRotationalConstraintDataAtomsVisitor> {
    fn from(data: &HkpRotationalConstraintDataAtoms) -> Self {
        vec![
            HkpRotationalConstraintDataAtomsVisitor::Rotations(data.rotations.clone()),
            HkpRotationalConstraintDataAtomsVisitor::Ang(data.ang.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpRotationalConstraintDataAtoms {
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
enum HkpRotationalConstraintDataAtomsVisitor {
    /// Visitor fields
    #[serde(rename = "rotations")]
    Rotations(SingleClass<HkpSetLocalRotationsConstraintAtom>),
    /// Visitor fields
    #[serde(rename = "ang")]
    Ang(SingleClass<HkpAngConstraintAtom>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpRotationalConstraintDataAtomsVisitor, "@name",
    ("rotations" => Rotations(SingleClass<HkpSetLocalRotationsConstraintAtom>)),
    ("ang" => Ang(SingleClass<HkpAngConstraintAtom>)),
}
