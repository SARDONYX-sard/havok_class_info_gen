//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPulleyConstraintDataAtoms`
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

/// `hkpPulleyConstraintDataAtoms`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: false
/// - signature: `0xb149e5a`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpPulleyConstraintDataAtoms {
    /// # C++ Class Fields Info
    /// -   name:`"translations"`
    /// -   type: `struct hkpSetLocalTranslationsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub translations: SingleClass<HkpSetLocalTranslationsConstraintAtom>,
    /// # C++ Class Fields Info
    /// -   name:`"pulley"`
    /// -   type: `struct hkpPulleyConstraintAtom`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub pulley: SingleClass<HkpPulleyConstraintAtom>,
}

impl Serialize for HkpPulleyConstraintDataAtoms {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpPulleyConstraintDataAtomsVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpPulleyConstraintDataAtoms {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpPulleyConstraintDataAtomsVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpPulleyConstraintDataAtomsVisitor>> for HkpPulleyConstraintDataAtoms {
    fn from(_values: Vec<HkpPulleyConstraintDataAtomsVisitor>) -> Self {
            let mut translations = None;
            let mut pulley = None;


        for _value in _values {
            match _value {
                HkpPulleyConstraintDataAtomsVisitor::Translations(m) => translations = Some(m),
                HkpPulleyConstraintDataAtomsVisitor::Pulley(m) => pulley = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            translations: translations.unwrap_or_default(),
            pulley: pulley.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpPulleyConstraintDataAtoms> for Vec<HkpPulleyConstraintDataAtomsVisitor> {
    fn from(data: &HkpPulleyConstraintDataAtoms) -> Self {
        vec![
            HkpPulleyConstraintDataAtomsVisitor::Translations(data.translations.clone()),
            HkpPulleyConstraintDataAtomsVisitor::Pulley(data.pulley.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpPulleyConstraintDataAtoms {
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
enum HkpPulleyConstraintDataAtomsVisitor {
    /// Visitor fields
    #[serde(rename = "translations")]
    Translations(SingleClass<HkpSetLocalTranslationsConstraintAtom>),
    /// Visitor fields
    #[serde(rename = "pulley")]
    Pulley(SingleClass<HkpPulleyConstraintAtom>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPulleyConstraintDataAtomsVisitor, "@name",
    ("translations" => Translations(SingleClass<HkpSetLocalTranslationsConstraintAtom>)),
    ("pulley" => Pulley(SingleClass<HkpPulleyConstraintAtom>)),
}
