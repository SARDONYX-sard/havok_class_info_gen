//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkCustomAttributes`
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

/// `hkCustomAttributes`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0xbff19005`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkCustomAttributes<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"attributes"`
    /// -   type: `hkSimpleArray<struct hkCustomAttributesAttribute>`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub attributes: HkArrayClass<HkCustomAttributesAttribute<'a>>,
}

impl Serialize for HkCustomAttributes<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkCustomAttributesVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkCustomAttributes<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkCustomAttributesVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkCustomAttributesVisitor<'a>>> for HkCustomAttributes<'a> {
    fn from(_values: Vec<HkCustomAttributesVisitor<'a>>) -> Self {
            let mut attributes = None;


        for _value in _values {
            match _value {
                HkCustomAttributesVisitor::Attributes(m) => attributes = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            attributes: attributes.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkCustomAttributes<'a>> for Vec<HkCustomAttributesVisitor<'a>> {
    fn from(data: &HkCustomAttributes<'a>) -> Self {
        vec![
            HkCustomAttributesVisitor::Attributes(data.attributes.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkCustomAttributes<'de> {
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
enum HkCustomAttributesVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "attributes")]
    Attributes(HkArrayClass<HkCustomAttributesAttribute<'a>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkCustomAttributesVisitor<'de>, "@name",
    ("attributes" => Attributes(HkArrayClass<HkCustomAttributesAttribute<'de>>)),
}