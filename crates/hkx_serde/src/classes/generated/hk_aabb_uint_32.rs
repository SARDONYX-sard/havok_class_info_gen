//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkAabbUint32`
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

/// `hkAabbUint32`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// - signature: `0x11e7c11`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkAabbUint32 {
    /// # C++ Class Fields Info
    /// -   name:`"min"`
    /// -   type: `hkUint32[3]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|ALIGN16`
    pub min: CStyleArray<[u32; 3]>,
    /// # C++ Class Fields Info
    /// -   name:`"expansionMin"`
    /// -   type: `hkUint8[3]`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub expansion_min: CStyleArray<[u8; 3]>,
    /// # C++ Class Fields Info
    /// -   name:`"expansionShift"`
    /// -   type: `hkUint8`
    /// - offset: 15
    /// -  flags: `FLAGS_NONE`
    pub expansion_shift: u8,
    /// # C++ Class Fields Info
    /// -   name:`"max"`
    /// -   type: `hkUint32[3]`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub max: CStyleArray<[u32; 3]>,
    /// # C++ Class Fields Info
    /// -   name:`"expansionMax"`
    /// -   type: `hkUint8[3]`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub expansion_max: CStyleArray<[u8; 3]>,
    /// # C++ Class Fields Info
    /// -   name:`"shapeKeyByte"`
    /// -   type: `hkUint8`
    /// - offset: 31
    /// -  flags: `FLAGS_NONE`
    pub shape_key_byte: u8,
}

impl Serialize for HkAabbUint32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkAabbUint32Visitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkAabbUint32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkAabbUint32Visitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkAabbUint32Visitor>> for HkAabbUint32 {
    fn from(_values: Vec<HkAabbUint32Visitor>) -> Self {
            let mut min = None;
            let mut expansion_min = None;
            let mut expansion_shift = None;
            let mut max = None;
            let mut expansion_max = None;
            let mut shape_key_byte = None;


        for _value in _values {
            match _value {
                HkAabbUint32Visitor::Min(m) => min = Some(m),
                HkAabbUint32Visitor::ExpansionMin(m) => expansion_min = Some(m),
                HkAabbUint32Visitor::ExpansionShift(m) => expansion_shift = Some(m),
                HkAabbUint32Visitor::Max(m) => max = Some(m),
                HkAabbUint32Visitor::ExpansionMax(m) => expansion_max = Some(m),
                HkAabbUint32Visitor::ShapeKeyByte(m) => shape_key_byte = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            min: min.unwrap_or_default(),
            expansion_min: expansion_min.unwrap_or_default(),
            expansion_shift: expansion_shift.unwrap_or_default().into_inner(),
            max: max.unwrap_or_default(),
            expansion_max: expansion_max.unwrap_or_default(),
            shape_key_byte: shape_key_byte.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkAabbUint32> for Vec<HkAabbUint32Visitor> {
    fn from(data: &HkAabbUint32) -> Self {
        vec![
            HkAabbUint32Visitor::Min(data.min.clone()),
            HkAabbUint32Visitor::ExpansionMin(data.expansion_min.clone()),
            HkAabbUint32Visitor::ExpansionShift(data.expansion_shift.into()),
            HkAabbUint32Visitor::Max(data.max.clone()),
            HkAabbUint32Visitor::ExpansionMax(data.expansion_max.clone()),
            HkAabbUint32Visitor::ShapeKeyByte(data.shape_key_byte.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkAabbUint32 {
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
enum HkAabbUint32Visitor {
    /// Visitor fields
    #[serde(rename = "min")]
    Min(CStyleArray<[u32; 3]>),
    /// Visitor fields
    #[serde(rename = "expansionMin")]
    ExpansionMin(CStyleArray<[u8; 3]>),
    /// Visitor fields
    #[serde(rename = "expansionShift")]
    ExpansionShift(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "max")]
    Max(CStyleArray<[u32; 3]>),
    /// Visitor fields
    #[serde(rename = "expansionMax")]
    ExpansionMax(CStyleArray<[u8; 3]>),
    /// Visitor fields
    #[serde(rename = "shapeKeyByte")]
    ShapeKeyByte(Primitive<u8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkAabbUint32Visitor, "@name",
    ("min" => Min(CStyleArray<[u32; 3]>)),
    ("expansionMin" => ExpansionMin(CStyleArray<[u8; 3]>)),
    ("expansionShift" => ExpansionShift(Primitive<u8>)),
    ("max" => Max(CStyleArray<[u32; 3]>)),
    ("expansionMax" => ExpansionMax(CStyleArray<[u8; 3]>)),
    ("shapeKeyByte" => ShapeKeyByte(Primitive<u8>)),
}
