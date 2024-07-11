//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbAttributeModifierAssignment`
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

/// `hkbAttributeModifierAssignment`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0x48b8ad52`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbAttributeModifierAssignment {
    /// # C++ Class Fields Info
    /// -   name:`"attributeIndex"`
    /// -   type: `hkInt32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub attribute_index: i32,
    /// # C++ Class Fields Info
    /// -   name:`"attributeValue"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub attribute_value: f32,
}

impl Serialize for HkbAttributeModifierAssignment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbAttributeModifierAssignmentVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbAttributeModifierAssignment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbAttributeModifierAssignmentVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbAttributeModifierAssignmentVisitor>> for HkbAttributeModifierAssignment {
    fn from(_values: Vec<HkbAttributeModifierAssignmentVisitor>) -> Self {
            let mut attribute_index = None;
            let mut attribute_value = None;


        for _value in _values {
            match _value {
                HkbAttributeModifierAssignmentVisitor::AttributeIndex(m) => attribute_index = Some(m),
                HkbAttributeModifierAssignmentVisitor::AttributeValue(m) => attribute_value = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            attribute_index: attribute_index.unwrap_or_default().into_inner(),
            attribute_value: attribute_value.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbAttributeModifierAssignment> for Vec<HkbAttributeModifierAssignmentVisitor> {
    fn from(data: &HkbAttributeModifierAssignment) -> Self {
        vec![
            HkbAttributeModifierAssignmentVisitor::AttributeIndex(data.attribute_index.into()),
            HkbAttributeModifierAssignmentVisitor::AttributeValue(data.attribute_value.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbAttributeModifierAssignment {
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
enum HkbAttributeModifierAssignmentVisitor {
    /// Visitor fields
    #[serde(rename = "attributeIndex")]
    AttributeIndex(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "attributeValue")]
    AttributeValue(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbAttributeModifierAssignmentVisitor, "@name",
    ("attributeIndex" => AttributeIndex(Primitive<i32>)),
    ("attributeValue" => AttributeValue(Primitive<f32>)),
}
