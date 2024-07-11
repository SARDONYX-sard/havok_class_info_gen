//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpBridgeConstraintAtom`
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

/// `hkpBridgeConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x87a4f31b`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpBridgeConstraintAtom<'a> {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub _type: AtomType,

    /// # C++ Class Fields Info
    /// -   name:`"buildJacobianFunc"`
    /// -   type: `void*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub build_jacobian_func: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"constraintData"`
    /// -   type: `struct hkpConstraintData*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE|NOT_OWNED`
    pub constraint_data: Cow<'a, str>,
}

impl Serialize for HkpBridgeConstraintAtom<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpBridgeConstraintAtomVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpBridgeConstraintAtom<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpBridgeConstraintAtomVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpBridgeConstraintAtomVisitor<'a>>> for HkpBridgeConstraintAtom<'a> {
    fn from(_values: Vec<HkpBridgeConstraintAtomVisitor<'a>>) -> Self {
            let mut _type = None;
            let mut build_jacobian_func = None;
            let mut constraint_data = None;


        for _value in _values {
            match _value {
                HkpBridgeConstraintAtomVisitor::Type(m) => _type = Some(m),
                HkpBridgeConstraintAtomVisitor::BuildJacobianFunc(m) => build_jacobian_func = Some(m),
                HkpBridgeConstraintAtomVisitor::ConstraintData(m) => constraint_data = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),
            build_jacobian_func: build_jacobian_func.unwrap_or_default().into_inner(),
            constraint_data: constraint_data.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpBridgeConstraintAtom<'a>> for Vec<HkpBridgeConstraintAtomVisitor<'a>> {
    fn from(data: &HkpBridgeConstraintAtom<'a>) -> Self {
        vec![
            HkpBridgeConstraintAtomVisitor::Type(data._type.clone().into()),
            HkpBridgeConstraintAtomVisitor::BuildJacobianFunc(data.build_jacobian_func.clone().into()),
            HkpBridgeConstraintAtomVisitor::ConstraintData(data.constraint_data.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpBridgeConstraintAtom<'de> {
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
enum HkpBridgeConstraintAtomVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// Visitor fields
    #[serde(rename = "buildJacobianFunc", skip_serializing)]
    BuildJacobianFunc(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "constraintData")]
    ConstraintData(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpBridgeConstraintAtomVisitor<'de>, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("buildJacobianFunc" => BuildJacobianFunc(Primitive<Cow<'de, str>>)),
    ("constraintData" => ConstraintData(Primitive<Cow<'de, str>>)),
}
