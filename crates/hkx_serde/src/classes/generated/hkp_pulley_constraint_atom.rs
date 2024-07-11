//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpPulleyConstraintAtom`
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

/// `hkpPulleyConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x94a08848`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpPulleyConstraintAtom {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub _type: AtomType,

    /// # C++ Class Fields Info
    /// -   name:`"fixedPivotAinWorld"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub fixed_pivot_ain_world: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"fixedPivotBinWorld"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub fixed_pivot_bin_world: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"ropeLength"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub rope_length: f32,
    /// # C++ Class Fields Info
    /// -   name:`"leverageOnBodyB"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub leverage_on_body_b: f32,
}

impl Serialize for HkpPulleyConstraintAtom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpPulleyConstraintAtomVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpPulleyConstraintAtom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpPulleyConstraintAtomVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpPulleyConstraintAtomVisitor>> for HkpPulleyConstraintAtom {
    fn from(_values: Vec<HkpPulleyConstraintAtomVisitor>) -> Self {
            let mut _type = None;
            let mut fixed_pivot_ain_world = None;
            let mut fixed_pivot_bin_world = None;
            let mut rope_length = None;
            let mut leverage_on_body_b = None;


        for _value in _values {
            match _value {
                HkpPulleyConstraintAtomVisitor::Type(m) => _type = Some(m),
                HkpPulleyConstraintAtomVisitor::FixedPivotAinWorld(m) => fixed_pivot_ain_world = Some(m),
                HkpPulleyConstraintAtomVisitor::FixedPivotBinWorld(m) => fixed_pivot_bin_world = Some(m),
                HkpPulleyConstraintAtomVisitor::RopeLength(m) => rope_length = Some(m),
                HkpPulleyConstraintAtomVisitor::LeverageOnBodyB(m) => leverage_on_body_b = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),
            fixed_pivot_ain_world: fixed_pivot_ain_world.unwrap_or_default().into_inner(),
            fixed_pivot_bin_world: fixed_pivot_bin_world.unwrap_or_default().into_inner(),
            rope_length: rope_length.unwrap_or_default().into_inner(),
            leverage_on_body_b: leverage_on_body_b.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpPulleyConstraintAtom> for Vec<HkpPulleyConstraintAtomVisitor> {
    fn from(data: &HkpPulleyConstraintAtom) -> Self {
        vec![
            HkpPulleyConstraintAtomVisitor::Type(data._type.clone().into()),
            HkpPulleyConstraintAtomVisitor::FixedPivotAinWorld(data.fixed_pivot_ain_world.into()),
            HkpPulleyConstraintAtomVisitor::FixedPivotBinWorld(data.fixed_pivot_bin_world.into()),
            HkpPulleyConstraintAtomVisitor::RopeLength(data.rope_length.into()),
            HkpPulleyConstraintAtomVisitor::LeverageOnBodyB(data.leverage_on_body_b.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpPulleyConstraintAtom {
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
enum HkpPulleyConstraintAtomVisitor {
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// Visitor fields
    #[serde(rename = "fixedPivotAinWorld")]
    FixedPivotAinWorld(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "fixedPivotBinWorld")]
    FixedPivotBinWorld(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "ropeLength")]
    RopeLength(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "leverageOnBodyB")]
    LeverageOnBodyB(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPulleyConstraintAtomVisitor, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("fixedPivotAinWorld" => FixedPivotAinWorld(Primitive<Vector4<f32>>)),
    ("fixedPivotBinWorld" => FixedPivotBinWorld(Primitive<Vector4<f32>>)),
    ("ropeLength" => RopeLength(Primitive<f32>)),
    ("leverageOnBodyB" => LeverageOnBodyB(Primitive<f32>)),
}
