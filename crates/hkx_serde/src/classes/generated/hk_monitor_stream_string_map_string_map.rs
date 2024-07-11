//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMonitorStreamStringMapStringMap`
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

/// `hkMonitorStreamStringMapStringMap`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x2c76ce16`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkMonitorStreamStringMapStringMap<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"id"`
    /// -   type: `hkUint64`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|ALIGN8`
    pub id: u64,
    /// # C++ Class Fields Info
    /// -   name:`"string"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub string: Cow<'a, str>,
}

impl Serialize for HkMonitorStreamStringMapStringMap<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkMonitorStreamStringMapStringMapVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkMonitorStreamStringMapStringMap<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkMonitorStreamStringMapStringMapVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkMonitorStreamStringMapStringMapVisitor<'a>>> for HkMonitorStreamStringMapStringMap<'a> {
    fn from(_values: Vec<HkMonitorStreamStringMapStringMapVisitor<'a>>) -> Self {
            let mut id = None;
            let mut string = None;


        for _value in _values {
            match _value {
                HkMonitorStreamStringMapStringMapVisitor::Id(m) => id = Some(m),
                HkMonitorStreamStringMapStringMapVisitor::String(m) => string = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            id: id.unwrap_or_default().into_inner(),
            string: string.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkMonitorStreamStringMapStringMap<'a>> for Vec<HkMonitorStreamStringMapStringMapVisitor<'a>> {
    fn from(data: &HkMonitorStreamStringMapStringMap<'a>) -> Self {
        vec![
            HkMonitorStreamStringMapStringMapVisitor::Id(data.id.into()),
            HkMonitorStreamStringMapStringMapVisitor::String(data.string.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkMonitorStreamStringMapStringMap<'de> {
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
enum HkMonitorStreamStringMapStringMapVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "id")]
    Id(Primitive<u64>),
    /// Visitor fields
    #[serde(rename = "string")]
    String(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMonitorStreamStringMapStringMapVisitor<'de>, "@name",
    ("id" => Id(Primitive<u64>)),
    ("string" => String(Primitive<Cow<'de, str>>)),
}
