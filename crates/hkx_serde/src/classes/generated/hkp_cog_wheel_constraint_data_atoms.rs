//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCogWheelConstraintDataAtoms`
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

/// `hkpCogWheelConstraintDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 160
/// -    vtable: false
/// - signature: `0xf855ba44`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpCogWheelConstraintDataAtoms {
    /// # C++ Class Fields Info
    /// -   name:`"transforms"`
    /// -   type: `struct hkpSetLocalTransformsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub transforms: SingleClass<HkpSetLocalTransformsConstraintAtom>,
    /// # C++ Class Fields Info
    /// -   name:`"cogWheels"`
    /// -   type: `struct hkpCogWheelConstraintAtom`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    pub cog_wheels: SingleClass<HkpCogWheelConstraintAtom>,
}

impl Serialize for HkpCogWheelConstraintDataAtoms {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpCogWheelConstraintDataAtomsVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpCogWheelConstraintDataAtoms {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpCogWheelConstraintDataAtomsVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpCogWheelConstraintDataAtomsVisitor>> for HkpCogWheelConstraintDataAtoms {
    fn from(_values: Vec<HkpCogWheelConstraintDataAtomsVisitor>) -> Self {
            let mut transforms = None;
            let mut cog_wheels = None;


        for _value in _values {
            match _value {
                HkpCogWheelConstraintDataAtomsVisitor::Transforms(m) => transforms = Some(m),
                HkpCogWheelConstraintDataAtomsVisitor::CogWheels(m) => cog_wheels = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            transforms: transforms.unwrap_or_default(),
            cog_wheels: cog_wheels.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpCogWheelConstraintDataAtoms> for Vec<HkpCogWheelConstraintDataAtomsVisitor> {
    fn from(data: &HkpCogWheelConstraintDataAtoms) -> Self {
        vec![
            HkpCogWheelConstraintDataAtomsVisitor::Transforms(data.transforms.clone()),
            HkpCogWheelConstraintDataAtomsVisitor::CogWheels(data.cog_wheels.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpCogWheelConstraintDataAtoms {
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
enum HkpCogWheelConstraintDataAtomsVisitor {
    /// Visitor fields
    #[serde(rename = "transforms")]
    Transforms(SingleClass<HkpSetLocalTransformsConstraintAtom>),
    /// Visitor fields
    #[serde(rename = "cogWheels")]
    CogWheels(SingleClass<HkpCogWheelConstraintAtom>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCogWheelConstraintDataAtomsVisitor, "@name",
    ("transforms" => Transforms(SingleClass<HkpSetLocalTransformsConstraintAtom>)),
    ("cogWheels" => CogWheels(SingleClass<HkpCogWheelConstraintAtom>)),
}
