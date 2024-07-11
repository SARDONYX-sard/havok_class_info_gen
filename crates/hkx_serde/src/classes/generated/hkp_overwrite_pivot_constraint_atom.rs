//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpOverwritePivotConstraintAtom`
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

/// `hkpOverwritePivotConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 4
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x1f11b467`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpOverwritePivotConstraintAtom {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub _type: AtomType,

    /// # C++ Class Fields Info
    /// -   name:`"copyToPivotBFromPivotA"`
    /// -   type: `hkUint8`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    pub copy_to_pivot_b_from_pivot_a: u8,
}

impl Serialize for HkpOverwritePivotConstraintAtom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpOverwritePivotConstraintAtomVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpOverwritePivotConstraintAtom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpOverwritePivotConstraintAtomVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpOverwritePivotConstraintAtomVisitor>> for HkpOverwritePivotConstraintAtom {
    fn from(_values: Vec<HkpOverwritePivotConstraintAtomVisitor>) -> Self {
            let mut _type = None;
            let mut copy_to_pivot_b_from_pivot_a = None;


        for _value in _values {
            match _value {
                HkpOverwritePivotConstraintAtomVisitor::Type(m) => _type = Some(m),
                HkpOverwritePivotConstraintAtomVisitor::CopyToPivotBFromPivotA(m) => copy_to_pivot_b_from_pivot_a = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),
            copy_to_pivot_b_from_pivot_a: copy_to_pivot_b_from_pivot_a.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpOverwritePivotConstraintAtom> for Vec<HkpOverwritePivotConstraintAtomVisitor> {
    fn from(data: &HkpOverwritePivotConstraintAtom) -> Self {
        vec![
            HkpOverwritePivotConstraintAtomVisitor::Type(data._type.clone().into()),
            HkpOverwritePivotConstraintAtomVisitor::CopyToPivotBFromPivotA(data.copy_to_pivot_b_from_pivot_a.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpOverwritePivotConstraintAtom {
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
enum HkpOverwritePivotConstraintAtomVisitor {
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// Visitor fields
    #[serde(rename = "copyToPivotBFromPivotA")]
    CopyToPivotBFromPivotA(Primitive<u8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpOverwritePivotConstraintAtomVisitor, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("copyToPivotBFromPivotA" => CopyToPivotBFromPivotA(Primitive<u8>)),
}
