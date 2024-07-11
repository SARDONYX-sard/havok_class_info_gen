//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSoftContactModifierConstraintAtom`
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

/// `hkpSoftContactModifierConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: false
/// -    parent: `hkpModifierConstraintAtom`/`0xb13fef1f`
/// - signature: `0xecb34e27`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpSoftContactModifierConstraintAtom<'a> {
    /// # C++ Parent class(`hkpModifierConstraintAtom` => parent: `hkpConstraintAtom`) field Info
    /// -   name:`"modifierAtomSize"`
    /// -   type: `hkUint16`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE|ALIGN16`
    pub modifier_atom_size: u16,
    /// # C++ Parent class(`hkpModifierConstraintAtom` => parent: `hkpConstraintAtom`) field Info
    /// -   name:`"childSize"`
    /// -   type: `hkUint16`
    /// - offset: 18
    /// -  flags: `FLAGS_NONE`
    pub child_size: u16,
    /// # C++ Parent class(`hkpModifierConstraintAtom` => parent: `hkpConstraintAtom`) field Info
    /// -   name:`"child"`
    /// -   type: `struct hkpConstraintAtom*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub child: Cow<'a, str>,
    /// # C++ Parent class(`hkpModifierConstraintAtom` => parent: `hkpConstraintAtom`) field Info
    /// -   name:`"pad"`
    /// -   type: `hkUint32[2]`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub pad: CStyleArray<[u32; 2]>,

    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub _type: AtomType,

    /// # C++ Class Fields Info
    /// -   name:`"tau"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub tau: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxAcceleration"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub max_acceleration: f32,
}

impl Serialize for HkpSoftContactModifierConstraintAtom<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpSoftContactModifierConstraintAtomVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpSoftContactModifierConstraintAtom<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpSoftContactModifierConstraintAtomVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpSoftContactModifierConstraintAtomVisitor<'a>>> for HkpSoftContactModifierConstraintAtom<'a> {
    fn from(_values: Vec<HkpSoftContactModifierConstraintAtomVisitor<'a>>) -> Self {
            let mut modifier_atom_size = None;
            let mut child_size = None;
            let mut child = None;
            let mut pad = None;
            let mut _type = None;
            let mut tau = None;
            let mut max_acceleration = None;


        for _value in _values {
            match _value {
                HkpSoftContactModifierConstraintAtomVisitor::ModifierAtomSize(m) => modifier_atom_size = Some(m),
                HkpSoftContactModifierConstraintAtomVisitor::ChildSize(m) => child_size = Some(m),
                HkpSoftContactModifierConstraintAtomVisitor::Child(m) => child = Some(m),
                HkpSoftContactModifierConstraintAtomVisitor::Pad(m) => pad = Some(m),
                HkpSoftContactModifierConstraintAtomVisitor::Type(m) => _type = Some(m),
                HkpSoftContactModifierConstraintAtomVisitor::Tau(m) => tau = Some(m),
                HkpSoftContactModifierConstraintAtomVisitor::MaxAcceleration(m) => max_acceleration = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            modifier_atom_size: modifier_atom_size.unwrap_or_default().into_inner(),
            child_size: child_size.unwrap_or_default().into_inner(),
            child: child.unwrap_or_default().into_inner(),
            pad: pad.unwrap_or_default(),
            _type: _type.unwrap_or_default().into_inner(),
            tau: tau.unwrap_or_default().into_inner(),
            max_acceleration: max_acceleration.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpSoftContactModifierConstraintAtom<'a>> for Vec<HkpSoftContactModifierConstraintAtomVisitor<'a>> {
    fn from(data: &HkpSoftContactModifierConstraintAtom<'a>) -> Self {
        vec![
            HkpSoftContactModifierConstraintAtomVisitor::ModifierAtomSize(data.modifier_atom_size.into()),
            HkpSoftContactModifierConstraintAtomVisitor::ChildSize(data.child_size.into()),
            HkpSoftContactModifierConstraintAtomVisitor::Child(data.child.clone().into()),
            HkpSoftContactModifierConstraintAtomVisitor::Pad(data.pad.clone()),
            HkpSoftContactModifierConstraintAtomVisitor::Type(data._type.clone().into()),
            HkpSoftContactModifierConstraintAtomVisitor::Tau(data.tau.into()),
            HkpSoftContactModifierConstraintAtomVisitor::MaxAcceleration(data.max_acceleration.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpSoftContactModifierConstraintAtom<'de> {
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
enum HkpSoftContactModifierConstraintAtomVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "modifierAtomSize")]
    ModifierAtomSize(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "childSize")]
    ChildSize(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "child")]
    Child(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "pad")]
    Pad(CStyleArray<[u32; 2]>),

    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// Visitor fields
    #[serde(rename = "tau")]
    Tau(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxAcceleration")]
    MaxAcceleration(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSoftContactModifierConstraintAtomVisitor<'de>, "@name",
    ("modifierAtomSize" => ModifierAtomSize(Primitive<u16>)),
    ("childSize" => ChildSize(Primitive<u16>)),
    ("child" => Child(Primitive<Cow<'de, str>>)),
    ("pad" => Pad(CStyleArray<[u32; 2]>)),
    ("type" => Type(Primitive<AtomType>)),
    ("tau" => Tau(Primitive<f32>)),
    ("maxAcceleration" => MaxAcceleration(Primitive<f32>)),
}
