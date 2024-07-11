//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpEntityExtendedListeners`
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

/// `hkpEntityExtendedListeners`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0xf557023c`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpEntityExtendedListeners<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"activationListeners"`
    /// -   type: `struct hkpEntitySmallArraySerializeOverrideType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub activation_listeners: SingleClass<HkpEntitySmallArraySerializeOverrideType<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"entityListeners"`
    /// -   type: `struct hkpEntitySmallArraySerializeOverrideType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub entity_listeners: SingleClass<HkpEntitySmallArraySerializeOverrideType<'a>>,
}

impl Serialize for HkpEntityExtendedListeners<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpEntityExtendedListenersVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpEntityExtendedListeners<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpEntityExtendedListenersVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpEntityExtendedListenersVisitor<'a>>> for HkpEntityExtendedListeners<'a> {
    fn from(_values: Vec<HkpEntityExtendedListenersVisitor<'a>>) -> Self {
            let mut activation_listeners = None;
            let mut entity_listeners = None;


        for _value in _values {
            match _value {
                HkpEntityExtendedListenersVisitor::ActivationListeners(m) => activation_listeners = Some(m),
                HkpEntityExtendedListenersVisitor::EntityListeners(m) => entity_listeners = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            activation_listeners: activation_listeners.unwrap_or_default(),
            entity_listeners: entity_listeners.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpEntityExtendedListeners<'a>> for Vec<HkpEntityExtendedListenersVisitor<'a>> {
    fn from(data: &HkpEntityExtendedListeners<'a>) -> Self {
        vec![
            HkpEntityExtendedListenersVisitor::ActivationListeners(data.activation_listeners.clone()),
            HkpEntityExtendedListenersVisitor::EntityListeners(data.entity_listeners.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpEntityExtendedListeners<'de> {
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
enum HkpEntityExtendedListenersVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "activationListeners", skip_serializing)]
    ActivationListeners(SingleClass<HkpEntitySmallArraySerializeOverrideType<'a>>),
    /// Visitor fields
    #[serde(rename = "entityListeners", skip_serializing)]
    EntityListeners(SingleClass<HkpEntitySmallArraySerializeOverrideType<'a>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpEntityExtendedListenersVisitor<'de>, "@name",
    ("activationListeners" => ActivationListeners(SingleClass<HkpEntitySmallArraySerializeOverrideType<'de>>)),
    ("entityListeners" => EntityListeners(SingleClass<HkpEntitySmallArraySerializeOverrideType<'de>>)),
}
