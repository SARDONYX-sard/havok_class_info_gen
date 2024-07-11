//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCenterOfMassChangerModifierConstraintAtom`
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

/// `hkpCenterOfMassChangerModifierConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 64
/// -    vtable: false
/// -    parent: `hkpModifierConstraintAtom`/`0xb13fef1f`
/// - signature: `0x1d7dbdd2`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpCenterOfMassChangerModifierConstraintAtom<'a> {
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
    /// -   name:`"displacementA"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub displacement_a: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"displacementB"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub displacement_b: Vector4<f32>,
}

impl Serialize for HkpCenterOfMassChangerModifierConstraintAtom<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpCenterOfMassChangerModifierConstraintAtomVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpCenterOfMassChangerModifierConstraintAtom<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpCenterOfMassChangerModifierConstraintAtomVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpCenterOfMassChangerModifierConstraintAtomVisitor<'a>>> for HkpCenterOfMassChangerModifierConstraintAtom<'a> {
    fn from(_values: Vec<HkpCenterOfMassChangerModifierConstraintAtomVisitor<'a>>) -> Self {
            let mut modifier_atom_size = None;
            let mut child_size = None;
            let mut child = None;
            let mut pad = None;
            let mut _type = None;
            let mut displacement_a = None;
            let mut displacement_b = None;


        for _value in _values {
            match _value {
                HkpCenterOfMassChangerModifierConstraintAtomVisitor::ModifierAtomSize(m) => modifier_atom_size = Some(m),
                HkpCenterOfMassChangerModifierConstraintAtomVisitor::ChildSize(m) => child_size = Some(m),
                HkpCenterOfMassChangerModifierConstraintAtomVisitor::Child(m) => child = Some(m),
                HkpCenterOfMassChangerModifierConstraintAtomVisitor::Pad(m) => pad = Some(m),
                HkpCenterOfMassChangerModifierConstraintAtomVisitor::Type(m) => _type = Some(m),
                HkpCenterOfMassChangerModifierConstraintAtomVisitor::DisplacementA(m) => displacement_a = Some(m),
                HkpCenterOfMassChangerModifierConstraintAtomVisitor::DisplacementB(m) => displacement_b = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            modifier_atom_size: modifier_atom_size.unwrap_or_default().into_inner(),
            child_size: child_size.unwrap_or_default().into_inner(),
            child: child.unwrap_or_default().into_inner(),
            pad: pad.unwrap_or_default(),
            _type: _type.unwrap_or_default().into_inner(),
            displacement_a: displacement_a.unwrap_or_default().into_inner(),
            displacement_b: displacement_b.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpCenterOfMassChangerModifierConstraintAtom<'a>> for Vec<HkpCenterOfMassChangerModifierConstraintAtomVisitor<'a>> {
    fn from(data: &HkpCenterOfMassChangerModifierConstraintAtom<'a>) -> Self {
        vec![
            HkpCenterOfMassChangerModifierConstraintAtomVisitor::ModifierAtomSize(data.modifier_atom_size.into()),
            HkpCenterOfMassChangerModifierConstraintAtomVisitor::ChildSize(data.child_size.into()),
            HkpCenterOfMassChangerModifierConstraintAtomVisitor::Child(data.child.clone().into()),
            HkpCenterOfMassChangerModifierConstraintAtomVisitor::Pad(data.pad.clone()),
            HkpCenterOfMassChangerModifierConstraintAtomVisitor::Type(data._type.clone().into()),
            HkpCenterOfMassChangerModifierConstraintAtomVisitor::DisplacementA(data.displacement_a.into()),
            HkpCenterOfMassChangerModifierConstraintAtomVisitor::DisplacementB(data.displacement_b.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpCenterOfMassChangerModifierConstraintAtom<'de> {
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
enum HkpCenterOfMassChangerModifierConstraintAtomVisitor<'a> {
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
    #[serde(rename = "displacementA")]
    DisplacementA(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "displacementB")]
    DisplacementB(Primitive<Vector4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCenterOfMassChangerModifierConstraintAtomVisitor<'de>, "@name",
    ("modifierAtomSize" => ModifierAtomSize(Primitive<u16>)),
    ("childSize" => ChildSize(Primitive<u16>)),
    ("child" => Child(Primitive<Cow<'de, str>>)),
    ("pad" => Pad(CStyleArray<[u32; 2]>)),
    ("type" => Type(Primitive<AtomType>)),
    ("displacementA" => DisplacementA(Primitive<Vector4<f32>>)),
    ("displacementB" => DisplacementB(Primitive<Vector4<f32>>)),
}
