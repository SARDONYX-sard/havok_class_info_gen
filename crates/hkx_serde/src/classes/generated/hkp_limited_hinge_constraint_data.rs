//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpLimitedHingeConstraintData`
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

/// `hkpLimitedHingeConstraintData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 256
/// -    vtable: true
/// -    parent: `hkpConstraintData`/`0x80559a4e`
/// - signature: `0x7c15bb6b`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpLimitedHingeConstraintData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"atoms"`
    /// -   type: `struct hkpLimitedHingeConstraintDataAtoms`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE|ALIGN16`
    pub atoms: SingleClass<HkpLimitedHingeConstraintDataAtoms<'a>>,
}

impl Serialize for HkpLimitedHingeConstraintData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpLimitedHingeConstraintDataVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpLimitedHingeConstraintData<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpLimitedHingeConstraintDataVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpLimitedHingeConstraintDataVisitor<'a>>> for HkpLimitedHingeConstraintData<'a> {
    fn from(_values: Vec<HkpLimitedHingeConstraintDataVisitor<'a>>) -> Self {
            let mut atoms = None;


        for _value in _values {
            match _value {
                HkpLimitedHingeConstraintDataVisitor::Atoms(m) => atoms = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            atoms: atoms.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpLimitedHingeConstraintData<'a>> for Vec<HkpLimitedHingeConstraintDataVisitor<'a>> {
    fn from(data: &HkpLimitedHingeConstraintData<'a>) -> Self {
        vec![
            HkpLimitedHingeConstraintDataVisitor::Atoms(data.atoms.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpLimitedHingeConstraintData<'de> {
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
enum HkpLimitedHingeConstraintDataVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "atoms")]
    Atoms(SingleClass<HkpLimitedHingeConstraintDataAtoms<'a>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpLimitedHingeConstraintDataVisitor<'de>, "@name",
    ("atoms" => Atoms(SingleClass<HkpLimitedHingeConstraintDataAtoms<'de>>)),
}
