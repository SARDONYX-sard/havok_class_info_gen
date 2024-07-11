//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSetLocalRotationsConstraintAtom`
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

/// `hkpSetLocalRotationsConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0xf81db8e`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpSetLocalRotationsConstraintAtom {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub _type: AtomType,

    /// # C++ Class Fields Info
    /// -   name:`"rotationA"`
    /// -   type: `hkRotation`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub rotation_a: Rotation<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"rotationB"`
    /// -   type: `hkRotation`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub rotation_b: Rotation<f32>,
}

impl Serialize for HkpSetLocalRotationsConstraintAtom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpSetLocalRotationsConstraintAtomVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpSetLocalRotationsConstraintAtom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpSetLocalRotationsConstraintAtomVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpSetLocalRotationsConstraintAtomVisitor>> for HkpSetLocalRotationsConstraintAtom {
    fn from(_values: Vec<HkpSetLocalRotationsConstraintAtomVisitor>) -> Self {
            let mut _type = None;
            let mut rotation_a = None;
            let mut rotation_b = None;


        for _value in _values {
            match _value {
                HkpSetLocalRotationsConstraintAtomVisitor::Type(m) => _type = Some(m),
                HkpSetLocalRotationsConstraintAtomVisitor::RotationA(m) => rotation_a = Some(m),
                HkpSetLocalRotationsConstraintAtomVisitor::RotationB(m) => rotation_b = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),
            rotation_a: rotation_a.unwrap_or_default().into_inner(),
            rotation_b: rotation_b.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpSetLocalRotationsConstraintAtom> for Vec<HkpSetLocalRotationsConstraintAtomVisitor> {
    fn from(data: &HkpSetLocalRotationsConstraintAtom) -> Self {
        vec![
            HkpSetLocalRotationsConstraintAtomVisitor::Type(data._type.clone().into()),
            HkpSetLocalRotationsConstraintAtomVisitor::RotationA(data.rotation_a.clone().into()),
            HkpSetLocalRotationsConstraintAtomVisitor::RotationB(data.rotation_b.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpSetLocalRotationsConstraintAtom {
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
enum HkpSetLocalRotationsConstraintAtomVisitor {
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// Visitor fields
    #[serde(rename = "rotationA")]
    RotationA(Primitive<Rotation<f32>>),
    /// Visitor fields
    #[serde(rename = "rotationB")]
    RotationB(Primitive<Rotation<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSetLocalRotationsConstraintAtomVisitor, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("rotationA" => RotationA(Primitive<Rotation<f32>>)),
    ("rotationB" => RotationB(Primitive<Rotation<f32>>)),
}
