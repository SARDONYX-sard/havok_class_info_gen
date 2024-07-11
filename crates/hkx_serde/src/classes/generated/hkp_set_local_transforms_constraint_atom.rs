//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSetLocalTransformsConstraintAtom`
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

/// `hkpSetLocalTransformsConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 144
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x6e2a5198`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpSetLocalTransformsConstraintAtom {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub _type: AtomType,

    /// # C++ Class Fields Info
    /// -   name:`"transformA"`
    /// -   type: `hkTransform`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub transform_a: Transform<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"transformB"`
    /// -   type: `hkTransform`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub transform_b: Transform<f32>,
}

impl Serialize for HkpSetLocalTransformsConstraintAtom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpSetLocalTransformsConstraintAtomVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpSetLocalTransformsConstraintAtom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpSetLocalTransformsConstraintAtomVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpSetLocalTransformsConstraintAtomVisitor>> for HkpSetLocalTransformsConstraintAtom {
    fn from(_values: Vec<HkpSetLocalTransformsConstraintAtomVisitor>) -> Self {
            let mut _type = None;
            let mut transform_a = None;
            let mut transform_b = None;


        for _value in _values {
            match _value {
                HkpSetLocalTransformsConstraintAtomVisitor::Type(m) => _type = Some(m),
                HkpSetLocalTransformsConstraintAtomVisitor::TransformA(m) => transform_a = Some(m),
                HkpSetLocalTransformsConstraintAtomVisitor::TransformB(m) => transform_b = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),
            transform_a: transform_a.unwrap_or_default().into_inner(),
            transform_b: transform_b.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpSetLocalTransformsConstraintAtom> for Vec<HkpSetLocalTransformsConstraintAtomVisitor> {
    fn from(data: &HkpSetLocalTransformsConstraintAtom) -> Self {
        vec![
            HkpSetLocalTransformsConstraintAtomVisitor::Type(data._type.clone().into()),
            HkpSetLocalTransformsConstraintAtomVisitor::TransformA(data.transform_a.clone().into()),
            HkpSetLocalTransformsConstraintAtomVisitor::TransformB(data.transform_b.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpSetLocalTransformsConstraintAtom {
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
enum HkpSetLocalTransformsConstraintAtomVisitor {
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// Visitor fields
    #[serde(rename = "transformA")]
    TransformA(Primitive<Transform<f32>>),
    /// Visitor fields
    #[serde(rename = "transformB")]
    TransformB(Primitive<Transform<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSetLocalTransformsConstraintAtomVisitor, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("transformA" => TransformA(Primitive<Transform<f32>>)),
    ("transformB" => TransformB(Primitive<Transform<f32>>)),
}
