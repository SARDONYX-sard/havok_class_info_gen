//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkaMeshBindingMapping`
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

/// `hkaMeshBindingMapping`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x48aceb75`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkaMeshBindingMapping {
    /// # C++ Class Fields Info
    /// -   name:`"mapping"`
    /// -   type: `hkArray<hkInt16>`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub mapping: HkArrayNum<i16>,
}

impl Serialize for HkaMeshBindingMapping {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkaMeshBindingMappingVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkaMeshBindingMapping {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkaMeshBindingMappingVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkaMeshBindingMappingVisitor>> for HkaMeshBindingMapping {
    fn from(_values: Vec<HkaMeshBindingMappingVisitor>) -> Self {
            let mut mapping = None;


        for _value in _values {
            match _value {
                HkaMeshBindingMappingVisitor::Mapping(m) => mapping = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mapping: mapping.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkaMeshBindingMapping> for Vec<HkaMeshBindingMappingVisitor> {
    fn from(data: &HkaMeshBindingMapping) -> Self {
        vec![
            HkaMeshBindingMappingVisitor::Mapping(data.mapping.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkaMeshBindingMapping {
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
enum HkaMeshBindingMappingVisitor {
    /// Visitor fields
    #[serde(rename = "mapping")]
    Mapping(HkArrayNum<i16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkaMeshBindingMappingVisitor, "@name",
    ("mapping" => Mapping(HkArrayNum<i16>)),
}
