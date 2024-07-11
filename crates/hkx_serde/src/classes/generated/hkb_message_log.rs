//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbMessageLog`
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

/// `hkbMessageLog`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0x26a196c5`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbMessageLog<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"messages"`
    /// -   type: `void*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub messages: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"maxMessages"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub max_messages: i32,
}

impl Serialize for HkbMessageLog<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbMessageLogVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbMessageLog<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbMessageLogVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbMessageLogVisitor<'a>>> for HkbMessageLog<'a> {
    fn from(_values: Vec<HkbMessageLogVisitor<'a>>) -> Self {
            let mut messages = None;
            let mut max_messages = None;


        for _value in _values {
            match _value {
                HkbMessageLogVisitor::Messages(m) => messages = Some(m),
                HkbMessageLogVisitor::MaxMessages(m) => max_messages = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            messages: messages.unwrap_or_default().into_inner(),
            max_messages: max_messages.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbMessageLog<'a>> for Vec<HkbMessageLogVisitor<'a>> {
    fn from(data: &HkbMessageLog<'a>) -> Self {
        vec![
            HkbMessageLogVisitor::Messages(data.messages.clone().into()),
            HkbMessageLogVisitor::MaxMessages(data.max_messages.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbMessageLog<'de> {
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
enum HkbMessageLogVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "messages", skip_serializing)]
    Messages(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "maxMessages", skip_serializing)]
    MaxMessages(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbMessageLogVisitor<'de>, "@name",
    ("messages" => Messages(Primitive<Cow<'de, str>>)),
    ("maxMessages" => MaxMessages(Primitive<i32>)),
}
