//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkLinkAttribute`
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

/// `hkLinkAttribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 1
/// -    vtable: false
/// - signature: `0x255d8164`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkLinkAttribute {
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum Link`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub _type: Link,
}

impl Serialize for HkLinkAttribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkLinkAttributeVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkLinkAttribute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkLinkAttributeVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkLinkAttributeVisitor>> for HkLinkAttribute {
    fn from(_values: Vec<HkLinkAttributeVisitor>) -> Self {
            let mut _type = None;


        for _value in _values {
            match _value {
                HkLinkAttributeVisitor::Type(m) => _type = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkLinkAttribute> for Vec<HkLinkAttributeVisitor> {
    fn from(data: &HkLinkAttribute) -> Self {
        vec![
            HkLinkAttributeVisitor::Type(data._type.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkLinkAttribute {
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
enum HkLinkAttributeVisitor {
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<Link>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkLinkAttributeVisitor, "@name",
    ("type" => Type(Primitive<Link>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum Link {
    #[serde(rename = "NONE")]
    #[default]
    None = 0,
    #[serde(rename = "DIRECT_LINK")]
    DirectLink = 1,
    #[serde(rename = "CHILD")]
    Child = 2,
    #[serde(rename = "MESH")]
    Mesh = 3,
    #[serde(rename = "PARENT_NAME")]
    ParentName = 4,
}
