//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkBitField`
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

/// `hkBitField`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0xda41bd9b`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkBitField {
    /// # C++ Class Fields Info
    /// -   name:`"words"`
    /// -   type: `hkArray<hkUint32>`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub words: HkArrayNum<u32>,
    /// # C++ Class Fields Info
    /// -   name:`"numBits"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub num_bits: i32,
}

impl Serialize for HkBitField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkBitFieldVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkBitField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkBitFieldVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkBitFieldVisitor>> for HkBitField {
    fn from(_values: Vec<HkBitFieldVisitor>) -> Self {
            let mut words = None;
            let mut num_bits = None;


        for _value in _values {
            match _value {
                HkBitFieldVisitor::Words(m) => words = Some(m),
                HkBitFieldVisitor::NumBits(m) => num_bits = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            words: words.unwrap_or_default(),
            num_bits: num_bits.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkBitField> for Vec<HkBitFieldVisitor> {
    fn from(data: &HkBitField) -> Self {
        vec![
            HkBitFieldVisitor::Words(data.words.clone()),
            HkBitFieldVisitor::NumBits(data.num_bits.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkBitField {
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
enum HkBitFieldVisitor {
    /// Visitor fields
    #[serde(rename = "words")]
    Words(HkArrayNum<u32>),
    /// Visitor fields
    #[serde(rename = "numBits")]
    NumBits(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkBitFieldVisitor, "@name",
    ("words" => Words(HkArrayNum<u32>)),
    ("numBits" => NumBits(Primitive<i32>)),
}
