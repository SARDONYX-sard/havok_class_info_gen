//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpMalleableConstraintData`
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

/// `hkpMalleableConstraintData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkpConstraintData`/`0x80559a4e`
/// - signature: `0x6748b2cf`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpMalleableConstraintData<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"constraintData"`
    /// -   type: `struct hkpConstraintData*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub constraint_data: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"atoms"`
    /// -   type: `struct hkpBridgeAtoms`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub atoms: SingleClass<HkpBridgeAtoms<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"strength"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub strength: f32,
}

impl Serialize for HkpMalleableConstraintData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpMalleableConstraintDataVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpMalleableConstraintData<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpMalleableConstraintDataVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpMalleableConstraintDataVisitor<'a>>> for HkpMalleableConstraintData<'a> {
    fn from(_values: Vec<HkpMalleableConstraintDataVisitor<'a>>) -> Self {
            let mut constraint_data = None;
            let mut atoms = None;
            let mut strength = None;


        for _value in _values {
            match _value {
                HkpMalleableConstraintDataVisitor::ConstraintData(m) => constraint_data = Some(m),
                HkpMalleableConstraintDataVisitor::Atoms(m) => atoms = Some(m),
                HkpMalleableConstraintDataVisitor::Strength(m) => strength = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            constraint_data: constraint_data.unwrap_or_default().into_inner(),
            atoms: atoms.unwrap_or_default(),
            strength: strength.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpMalleableConstraintData<'a>> for Vec<HkpMalleableConstraintDataVisitor<'a>> {
    fn from(data: &HkpMalleableConstraintData<'a>) -> Self {
        vec![
            HkpMalleableConstraintDataVisitor::ConstraintData(data.constraint_data.clone().into()),
            HkpMalleableConstraintDataVisitor::Atoms(data.atoms.clone()),
            HkpMalleableConstraintDataVisitor::Strength(data.strength.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpMalleableConstraintData<'de> {
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
enum HkpMalleableConstraintDataVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "constraintData")]
    ConstraintData(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "atoms")]
    Atoms(SingleClass<HkpBridgeAtoms<'a>>),
    /// Visitor fields
    #[serde(rename = "strength")]
    Strength(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMalleableConstraintDataVisitor<'de>, "@name",
    ("constraintData" => ConstraintData(Primitive<Cow<'de, str>>)),
    ("atoms" => Atoms(SingleClass<HkpBridgeAtoms<'de>>)),
    ("strength" => Strength(Primitive<f32>)),
}
