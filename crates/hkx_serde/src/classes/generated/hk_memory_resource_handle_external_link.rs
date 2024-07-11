//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkMemoryResourceHandleExternalLink`
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

/// `hkMemoryResourceHandleExternalLink`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0x3144d17c`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkMemoryResourceHandleExternalLink<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"memberName"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub member_name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"externalId"`
    /// -   type: `hkStringPtr`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub external_id: Cow<'a, str>,
}

impl Serialize for HkMemoryResourceHandleExternalLink<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkMemoryResourceHandleExternalLinkVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkMemoryResourceHandleExternalLink<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkMemoryResourceHandleExternalLinkVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkMemoryResourceHandleExternalLinkVisitor<'a>>> for HkMemoryResourceHandleExternalLink<'a> {
    fn from(_values: Vec<HkMemoryResourceHandleExternalLinkVisitor<'a>>) -> Self {
            let mut member_name = None;
            let mut external_id = None;


        for _value in _values {
            match _value {
                HkMemoryResourceHandleExternalLinkVisitor::MemberName(m) => member_name = Some(m),
                HkMemoryResourceHandleExternalLinkVisitor::ExternalId(m) => external_id = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            member_name: member_name.unwrap_or_default().into_inner(),
            external_id: external_id.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkMemoryResourceHandleExternalLink<'a>> for Vec<HkMemoryResourceHandleExternalLinkVisitor<'a>> {
    fn from(data: &HkMemoryResourceHandleExternalLink<'a>) -> Self {
        vec![
            HkMemoryResourceHandleExternalLinkVisitor::MemberName(data.member_name.clone().into()),
            HkMemoryResourceHandleExternalLinkVisitor::ExternalId(data.external_id.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkMemoryResourceHandleExternalLink<'de> {
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
enum HkMemoryResourceHandleExternalLinkVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memberName")]
    MemberName(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "externalId")]
    ExternalId(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkMemoryResourceHandleExternalLinkVisitor<'de>, "@name",
    ("memberName" => MemberName(Primitive<Cow<'de, str>>)),
    ("externalId" => ExternalId(Primitive<Cow<'de, str>>)),
}
