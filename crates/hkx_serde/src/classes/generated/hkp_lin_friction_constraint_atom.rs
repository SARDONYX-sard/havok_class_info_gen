//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpLinFrictionConstraintAtom`
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

/// `hkpLinFrictionConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x3e94ef7c`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpLinFrictionConstraintAtom {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub _type: AtomType,

    /// # C++ Class Fields Info
    /// -   name:`"isEnabled"`
    /// -   type: `hkUint8`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    pub is_enabled: u8,
    /// # C++ Class Fields Info
    /// -   name:`"frictionAxis"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    pub friction_axis: u8,
    /// # C++ Class Fields Info
    /// -   name:`"maxFrictionForce"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub max_friction_force: f32,
}

impl Serialize for HkpLinFrictionConstraintAtom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpLinFrictionConstraintAtomVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpLinFrictionConstraintAtom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpLinFrictionConstraintAtomVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpLinFrictionConstraintAtomVisitor>> for HkpLinFrictionConstraintAtom {
    fn from(_values: Vec<HkpLinFrictionConstraintAtomVisitor>) -> Self {
            let mut _type = None;
            let mut is_enabled = None;
            let mut friction_axis = None;
            let mut max_friction_force = None;


        for _value in _values {
            match _value {
                HkpLinFrictionConstraintAtomVisitor::Type(m) => _type = Some(m),
                HkpLinFrictionConstraintAtomVisitor::IsEnabled(m) => is_enabled = Some(m),
                HkpLinFrictionConstraintAtomVisitor::FrictionAxis(m) => friction_axis = Some(m),
                HkpLinFrictionConstraintAtomVisitor::MaxFrictionForce(m) => max_friction_force = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),
            is_enabled: is_enabled.unwrap_or_default().into_inner(),
            friction_axis: friction_axis.unwrap_or_default().into_inner(),
            max_friction_force: max_friction_force.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpLinFrictionConstraintAtom> for Vec<HkpLinFrictionConstraintAtomVisitor> {
    fn from(data: &HkpLinFrictionConstraintAtom) -> Self {
        vec![
            HkpLinFrictionConstraintAtomVisitor::Type(data._type.clone().into()),
            HkpLinFrictionConstraintAtomVisitor::IsEnabled(data.is_enabled.into()),
            HkpLinFrictionConstraintAtomVisitor::FrictionAxis(data.friction_axis.into()),
            HkpLinFrictionConstraintAtomVisitor::MaxFrictionForce(data.max_friction_force.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpLinFrictionConstraintAtom {
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
enum HkpLinFrictionConstraintAtomVisitor {
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// Visitor fields
    #[serde(rename = "isEnabled")]
    IsEnabled(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "frictionAxis")]
    FrictionAxis(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "maxFrictionForce")]
    MaxFrictionForce(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpLinFrictionConstraintAtomVisitor, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("isEnabled" => IsEnabled(Primitive<u8>)),
    ("frictionAxis" => FrictionAxis(Primitive<u8>)),
    ("maxFrictionForce" => MaxFrictionForce(Primitive<f32>)),
}
