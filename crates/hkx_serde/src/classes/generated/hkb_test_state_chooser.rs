//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbTestStateChooser`
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

/// `hkbTestStateChooser`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: true
/// -    parent: `hkbStateChooser`/`0xda8c7d7d`
/// - signature: `0xc0fcc436`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbTestStateChooser<'a> {
    // C++ Parent class(`hkbStateChooser` => parent: `hkReferencedObject`) has no fields
    //
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub mem_size_and_flags: u16,
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub reference_count: i16,

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// # C++ Class Fields Info
    /// -   name:`"int"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub int: i32,
    /// # C++ Class Fields Info
    /// -   name:`"real"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub real: f32,
    /// # C++ Class Fields Info
    /// -   name:`"string"`
    /// -   type: `hkStringPtr`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub string: Cow<'a, str>,
}

impl Serialize for HkbTestStateChooser<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbTestStateChooserVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbTestStateChooser<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbTestStateChooserVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbTestStateChooserVisitor<'a>>> for HkbTestStateChooser<'a> {
    fn from(_values: Vec<HkbTestStateChooserVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut int = None;
            let mut real = None;
            let mut string = None;


        for _value in _values {
            match _value {
                HkbTestStateChooserVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbTestStateChooserVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbTestStateChooserVisitor::Int(m) => int = Some(m),
                HkbTestStateChooserVisitor::Real(m) => real = Some(m),
                HkbTestStateChooserVisitor::String(m) => string = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            int: int.unwrap_or_default().into_inner(),
            real: real.unwrap_or_default().into_inner(),
            string: string.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbTestStateChooser<'a>> for Vec<HkbTestStateChooserVisitor<'a>> {
    fn from(data: &HkbTestStateChooser<'a>) -> Self {
        vec![
            HkbTestStateChooserVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbTestStateChooserVisitor::ReferenceCount(data.reference_count.into()),
            HkbTestStateChooserVisitor::Int(data.int.into()),
            HkbTestStateChooserVisitor::Real(data.real.into()),
            HkbTestStateChooserVisitor::String(data.string.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbTestStateChooser<'de> {
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
enum HkbTestStateChooserVisitor<'a> {
    // C++ Parent class(`hkbStateChooser` => parent: `hkReferencedObject`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "int")]
    Int(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "real")]
    Real(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "string")]
    String(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbTestStateChooserVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("int" => Int(Primitive<i32>)),
    ("real" => Real(Primitive<f32>)),
    ("string" => String(Primitive<Cow<'de, str>>)),
}
