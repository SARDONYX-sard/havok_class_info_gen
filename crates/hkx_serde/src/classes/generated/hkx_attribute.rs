//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxAttribute`
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

/// `hkxAttribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0x7375cae3`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkxAttribute<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub name: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"value"`
    /// -   type: `struct hkReferencedObject*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub value: Cow<'a, str>,
}

impl Serialize for HkxAttribute<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkxAttributeVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkxAttribute<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkxAttributeVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkxAttributeVisitor<'a>>> for HkxAttribute<'a> {
    fn from(_values: Vec<HkxAttributeVisitor<'a>>) -> Self {
            let mut name = None;
            let mut value = None;


        for _value in _values {
            match _value {
                HkxAttributeVisitor::Name(m) => name = Some(m),
                HkxAttributeVisitor::Value(m) => value = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            name: name.unwrap_or_default().into_inner(),
            value: value.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkxAttribute<'a>> for Vec<HkxAttributeVisitor<'a>> {
    fn from(data: &HkxAttribute<'a>) -> Self {
        vec![
            HkxAttributeVisitor::Name(data.name.clone().into()),
            HkxAttributeVisitor::Value(data.value.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkxAttribute<'de> {
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
enum HkxAttributeVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "value")]
    Value(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxAttributeVisitor<'de>, "@name",
    ("name" => Name(Primitive<Cow<'de, str>>)),
    ("value" => Value(Primitive<Cow<'de, str>>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum Hint {
    #[serde(rename = "HINT_NONE")]
    #[default]
    HintNone = 0,
    #[serde(rename = "HINT_IGNORE")]
    HintIgnore = 1,
    #[serde(rename = "HINT_TRANSFORM")]
    HintTransform = 2,
    #[serde(rename = "HINT_SCALE")]
    HintScale = 4,
    #[serde(rename = "HINT_TRANSFORM_AND_SCALE")]
    HintTransformAndScale = 6,
    #[serde(rename = "HINT_FLIP")]
    HintFlip = 8,
}
