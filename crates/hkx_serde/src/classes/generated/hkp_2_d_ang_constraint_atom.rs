//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkp2dAngConstraintAtom`
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

/// `hkp2dAngConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 4
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0xdcdb8b8b`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Hkp2DAngConstraintAtom {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub _type: AtomType,

    /// # C++ Class Fields Info
    /// -   name:`"freeRotationAxis"`
    /// -   type: `hkUint8`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    pub free_rotation_axis: u8,
}

impl Serialize for Hkp2DAngConstraintAtom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<Hkp2DAngConstraintAtomVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Hkp2DAngConstraintAtom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<Hkp2DAngConstraintAtomVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<Hkp2DAngConstraintAtomVisitor>> for Hkp2DAngConstraintAtom {
    fn from(_values: Vec<Hkp2DAngConstraintAtomVisitor>) -> Self {
            let mut _type = None;
            let mut free_rotation_axis = None;


        for _value in _values {
            match _value {
                Hkp2DAngConstraintAtomVisitor::Type(m) => _type = Some(m),
                Hkp2DAngConstraintAtomVisitor::FreeRotationAxis(m) => free_rotation_axis = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),
            free_rotation_axis: free_rotation_axis.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&Hkp2DAngConstraintAtom> for Vec<Hkp2DAngConstraintAtomVisitor> {
    fn from(data: &Hkp2DAngConstraintAtom) -> Self {
        vec![
            Hkp2DAngConstraintAtomVisitor::Type(data._type.clone().into()),
            Hkp2DAngConstraintAtomVisitor::FreeRotationAxis(data.free_rotation_axis.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for Hkp2DAngConstraintAtom {
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
enum Hkp2DAngConstraintAtomVisitor {
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// Visitor fields
    #[serde(rename = "freeRotationAxis")]
    FreeRotationAxis(Primitive<u8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    Hkp2DAngConstraintAtomVisitor, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("freeRotationAxis" => FreeRotationAxis(Primitive<u8>)),
}
