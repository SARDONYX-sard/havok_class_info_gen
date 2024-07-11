//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkAabbHalf`
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

/// `hkAabbHalf`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x1d716a17`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkAabbHalf {
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `hkUint16[6]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub data: CStyleArray<[u16; 6]>,
    /// # C++ Class Fields Info
    /// -   name:`"extras"`
    /// -   type: `hkUint16[2]`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub extras: CStyleArray<[u16; 2]>,
}

impl Serialize for HkAabbHalf {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkAabbHalfVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkAabbHalf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkAabbHalfVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkAabbHalfVisitor>> for HkAabbHalf {
    fn from(_values: Vec<HkAabbHalfVisitor>) -> Self {
            let mut data = None;
            let mut extras = None;


        for _value in _values {
            match _value {
                HkAabbHalfVisitor::Data(m) => data = Some(m),
                HkAabbHalfVisitor::Extras(m) => extras = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            data: data.unwrap_or_default(),
            extras: extras.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkAabbHalf> for Vec<HkAabbHalfVisitor> {
    fn from(data: &HkAabbHalf) -> Self {
        vec![
            HkAabbHalfVisitor::Data(data.data.clone()),
            HkAabbHalfVisitor::Extras(data.extras.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkAabbHalf {
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
enum HkAabbHalfVisitor {
    /// Visitor fields
    #[serde(rename = "data")]
    Data(CStyleArray<[u16; 6]>),
    /// Visitor fields
    #[serde(rename = "extras")]
    Extras(CStyleArray<[u16; 2]>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkAabbHalfVisitor, "@name",
    ("data" => Data(CStyleArray<[u16; 6]>)),
    ("extras" => Extras(CStyleArray<[u16; 2]>)),
}
