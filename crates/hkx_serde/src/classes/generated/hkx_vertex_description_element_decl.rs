//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxVertexDescriptionElementDecl`
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

/// `hkxVertexDescriptionElementDecl`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x483a429b`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkxVertexDescriptionElementDecl {
    /// # C++ Class Fields Info
    /// -   name:`"byteOffset"`
    /// -   type: `hkUint32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub byte_offset: u32,
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum DataType`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub _type: DataType,
    /// # C++ Class Fields Info
    /// -   name:`"usage"`
    /// -   type: `enum DataUsage`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    pub usage: DataUsage,
    /// # C++ Class Fields Info
    /// -   name:`"byteStride"`
    /// -   type: `hkUint32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub byte_stride: u32,
    /// # C++ Class Fields Info
    /// -   name:`"numElements"`
    /// -   type: `hkUint8`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub num_elements: u8,
}

impl Serialize for HkxVertexDescriptionElementDecl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkxVertexDescriptionElementDeclVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkxVertexDescriptionElementDecl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkxVertexDescriptionElementDeclVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkxVertexDescriptionElementDeclVisitor>> for HkxVertexDescriptionElementDecl {
    fn from(_values: Vec<HkxVertexDescriptionElementDeclVisitor>) -> Self {
            let mut byte_offset = None;
            let mut _type = None;
            let mut usage = None;
            let mut byte_stride = None;
            let mut num_elements = None;


        for _value in _values {
            match _value {
                HkxVertexDescriptionElementDeclVisitor::ByteOffset(m) => byte_offset = Some(m),
                HkxVertexDescriptionElementDeclVisitor::Type(m) => _type = Some(m),
                HkxVertexDescriptionElementDeclVisitor::Usage(m) => usage = Some(m),
                HkxVertexDescriptionElementDeclVisitor::ByteStride(m) => byte_stride = Some(m),
                HkxVertexDescriptionElementDeclVisitor::NumElements(m) => num_elements = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            byte_offset: byte_offset.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            usage: usage.unwrap_or_default().into_inner(),
            byte_stride: byte_stride.unwrap_or_default().into_inner(),
            num_elements: num_elements.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkxVertexDescriptionElementDecl> for Vec<HkxVertexDescriptionElementDeclVisitor> {
    fn from(data: &HkxVertexDescriptionElementDecl) -> Self {
        vec![
            HkxVertexDescriptionElementDeclVisitor::ByteOffset(data.byte_offset.into()),
            HkxVertexDescriptionElementDeclVisitor::Type(data._type.clone().into()),
            HkxVertexDescriptionElementDeclVisitor::Usage(data.usage.clone().into()),
            HkxVertexDescriptionElementDeclVisitor::ByteStride(data.byte_stride.into()),
            HkxVertexDescriptionElementDeclVisitor::NumElements(data.num_elements.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkxVertexDescriptionElementDecl {
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
enum HkxVertexDescriptionElementDeclVisitor {
    /// Visitor fields
    #[serde(rename = "byteOffset")]
    ByteOffset(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<DataType>),
    /// Visitor fields
    #[serde(rename = "usage")]
    Usage(Primitive<DataUsage>),
    /// Visitor fields
    #[serde(rename = "byteStride")]
    ByteStride(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "numElements")]
    NumElements(Primitive<u8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxVertexDescriptionElementDeclVisitor, "@name",
    ("byteOffset" => ByteOffset(Primitive<u32>)),
    ("type" => Type(Primitive<DataType>)),
    ("usage" => Usage(Primitive<DataUsage>)),
    ("byteStride" => ByteStride(Primitive<u32>)),
    ("numElements" => NumElements(Primitive<u8>)),
}
