//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbEventBase`
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

/// `hkbEventBase`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0x76bddb31`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbEventBase<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"id"`
    /// -   type: `hkInt32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub id: i32,
    /// # C++ Class Fields Info
    /// -   name:`"payload"`
    /// -   type: `struct hkbEventPayload*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub payload: Cow<'a, str>,
}

impl Serialize for HkbEventBase<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbEventBaseVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbEventBase<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbEventBaseVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbEventBaseVisitor<'a>>> for HkbEventBase<'a> {
    fn from(_values: Vec<HkbEventBaseVisitor<'a>>) -> Self {
            let mut id = None;
            let mut payload = None;


        for _value in _values {
            match _value {
                HkbEventBaseVisitor::Id(m) => id = Some(m),
                HkbEventBaseVisitor::Payload(m) => payload = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            id: id.unwrap_or_default().into_inner(),
            payload: payload.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbEventBase<'a>> for Vec<HkbEventBaseVisitor<'a>> {
    fn from(data: &HkbEventBase<'a>) -> Self {
        vec![
            HkbEventBaseVisitor::Id(data.id.into()),
            HkbEventBaseVisitor::Payload(data.payload.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbEventBase<'de> {
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
enum HkbEventBaseVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "id")]
    Id(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "payload")]
    Payload(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbEventBaseVisitor<'de>, "@name",
    ("id" => Id(Primitive<i32>)),
    ("payload" => Payload(Primitive<Cow<'de, str>>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum SystemEventIds {
    #[serde(rename = "EVENT_ID_NULL")]
    #[default]
    EventIdNull = -1,
}
