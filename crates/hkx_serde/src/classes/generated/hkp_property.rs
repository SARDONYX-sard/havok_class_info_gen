//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpProperty`
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

/// `hkpProperty`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x9ce308e9`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpProperty {
    /// # C++ Class Fields Info
    /// -   name:`"key"`
    /// -   type: `hkUint32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub key: u32,
    /// # C++ Class Fields Info
    /// -   name:`"alignmentPadding"`
    /// -   type: `hkUint32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub alignment_padding: u32,
    /// # C++ Class Fields Info
    /// -   name:`"value"`
    /// -   type: `struct hkpPropertyValue`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub value: SingleClass<HkpPropertyValue>,
}

impl Serialize for HkpProperty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpPropertyVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpProperty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpPropertyVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpPropertyVisitor>> for HkpProperty {
    fn from(_values: Vec<HkpPropertyVisitor>) -> Self {
            let mut key = None;
            let mut alignment_padding = None;
            let mut value = None;


        for _value in _values {
            match _value {
                HkpPropertyVisitor::Key(m) => key = Some(m),
                HkpPropertyVisitor::AlignmentPadding(m) => alignment_padding = Some(m),
                HkpPropertyVisitor::Value(m) => value = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            key: key.unwrap_or_default().into_inner(),
            alignment_padding: alignment_padding.unwrap_or_default().into_inner(),
            value: value.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpProperty> for Vec<HkpPropertyVisitor> {
    fn from(data: &HkpProperty) -> Self {
        vec![
            HkpPropertyVisitor::Key(data.key.into()),
            HkpPropertyVisitor::AlignmentPadding(data.alignment_padding.into()),
            HkpPropertyVisitor::Value(data.value.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpProperty {
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
enum HkpPropertyVisitor {
    /// Visitor fields
    #[serde(rename = "key")]
    Key(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "alignmentPadding")]
    AlignmentPadding(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "value")]
    Value(SingleClass<HkpPropertyValue>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpPropertyVisitor, "@name",
    ("key" => Key(Primitive<u32>)),
    ("alignmentPadding" => AlignmentPadding(Primitive<u32>)),
    ("value" => Value(SingleClass<HkpPropertyValue>)),
}
