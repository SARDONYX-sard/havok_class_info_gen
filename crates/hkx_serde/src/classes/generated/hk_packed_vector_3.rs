//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkPackedVector3`
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

/// `hkPackedVector3`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0x9c16df5b`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkPackedVector3 {
    /// # C++ Class Fields Info
    /// -   name:`"values"`
    /// -   type: `hkInt16[4]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub values: CStyleArray<[i16; 4]>,
}

impl Serialize for HkPackedVector3 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkPackedVector3Visitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkPackedVector3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkPackedVector3Visitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkPackedVector3Visitor>> for HkPackedVector3 {
    fn from(_values: Vec<HkPackedVector3Visitor>) -> Self {
            let mut values = None;


        for _value in _values {
            match _value {
                HkPackedVector3Visitor::Values(m) => values = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            values: values.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkPackedVector3> for Vec<HkPackedVector3Visitor> {
    fn from(data: &HkPackedVector3) -> Self {
        vec![
            HkPackedVector3Visitor::Values(data.values.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkPackedVector3 {
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
enum HkPackedVector3Visitor {
    /// Visitor fields
    #[serde(rename = "values")]
    Values(CStyleArray<[i16; 4]>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkPackedVector3Visitor, "@name",
    ("values" => Values(CStyleArray<[i16; 4]>)),
}
