//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpSingleShapeContainer`
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

/// `hkpSingleShapeContainer`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: true
/// -    parent: `hkpShapeContainer`/`0xe0708a00`
/// - signature: `0x73aa1d38`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpSingleShapeContainer<'a> {
    // C++ Parent class(`hkpShapeContainer` => parent: `None`) has no fields
    //
    /// # C++ Class Fields Info
    /// -   name:`"childShape"`
    /// -   type: `struct hkpShape*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub child_shape: Cow<'a, str>,
}

impl Serialize for HkpSingleShapeContainer<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpSingleShapeContainerVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpSingleShapeContainer<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpSingleShapeContainerVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpSingleShapeContainerVisitor<'a>>> for HkpSingleShapeContainer<'a> {
    fn from(_values: Vec<HkpSingleShapeContainerVisitor<'a>>) -> Self {
            let mut child_shape = None;


        for _value in _values {
            match _value {
                HkpSingleShapeContainerVisitor::ChildShape(m) => child_shape = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            child_shape: child_shape.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpSingleShapeContainer<'a>> for Vec<HkpSingleShapeContainerVisitor<'a>> {
    fn from(data: &HkpSingleShapeContainer<'a>) -> Self {
        vec![
            HkpSingleShapeContainerVisitor::ChildShape(data.child_shape.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpSingleShapeContainer<'de> {
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
enum HkpSingleShapeContainerVisitor<'a> {
    // C++ Parent class(`hkpShapeContainer` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "childShape")]
    ChildShape(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpSingleShapeContainerVisitor<'de>, "@name",
    ("childShape" => ChildShape(Primitive<Cow<'de, str>>)),
}
