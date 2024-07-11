//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSetLocalTranslationsConstraintAtom`
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

/// `hkpSetLocalTranslationsConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x5cbfcf4a`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpSetLocalTranslationsConstraintAtom {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub _type: AtomType,

    /// # C++ Class Fields Info
    /// -   name:`"translationA"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub translation_a: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"translationB"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub translation_b: Vector4<f32>,
}

impl Serialize for HkpSetLocalTranslationsConstraintAtom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpSetLocalTranslationsConstraintAtomVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpSetLocalTranslationsConstraintAtom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpSetLocalTranslationsConstraintAtomVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpSetLocalTranslationsConstraintAtomVisitor>> for HkpSetLocalTranslationsConstraintAtom {
    fn from(_values: Vec<HkpSetLocalTranslationsConstraintAtomVisitor>) -> Self {
            let mut _type = None;
            let mut translation_a = None;
            let mut translation_b = None;


        for _value in _values {
            match _value {
                HkpSetLocalTranslationsConstraintAtomVisitor::Type(m) => _type = Some(m),
                HkpSetLocalTranslationsConstraintAtomVisitor::TranslationA(m) => translation_a = Some(m),
                HkpSetLocalTranslationsConstraintAtomVisitor::TranslationB(m) => translation_b = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),
            translation_a: translation_a.unwrap_or_default().into_inner(),
            translation_b: translation_b.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpSetLocalTranslationsConstraintAtom> for Vec<HkpSetLocalTranslationsConstraintAtomVisitor> {
    fn from(data: &HkpSetLocalTranslationsConstraintAtom) -> Self {
        vec![
            HkpSetLocalTranslationsConstraintAtomVisitor::Type(data._type.clone().into()),
            HkpSetLocalTranslationsConstraintAtomVisitor::TranslationA(data.translation_a.into()),
            HkpSetLocalTranslationsConstraintAtomVisitor::TranslationB(data.translation_b.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpSetLocalTranslationsConstraintAtom {
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
enum HkpSetLocalTranslationsConstraintAtomVisitor {
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// Visitor fields
    #[serde(rename = "translationA")]
    TranslationA(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "translationB")]
    TranslationB(Primitive<Vector4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSetLocalTranslationsConstraintAtomVisitor, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("translationA" => TranslationA(Primitive<Vector4<f32>>)),
    ("translationB" => TranslationB(Primitive<Vector4<f32>>)),
}
