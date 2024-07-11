//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkVertexFormatElement`
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

/// `hkVertexFormatElement`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0x54867cbf`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkVertexFormatElement {
    /// # C++ Class Fields Info
    /// -   name:`"dataType"`
    /// -   type: `enum ComponentType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub data_type: ComponentType,
    /// # C++ Class Fields Info
    /// -   name:`"numValues"`
    /// -   type: `hkUint8`
    /// - offset: 1
    /// -  flags: `FLAGS_NONE`
    pub num_values: u8,
    /// # C++ Class Fields Info
    /// -   name:`"usage"`
    /// -   type: `enum ComponentUsage`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    pub usage: ComponentUsage,
    /// # C++ Class Fields Info
    /// -   name:`"subUsage"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    pub sub_usage: u8,
    /// # C++ Class Fields Info
    /// -   name:`"flags"`
    /// -   type: `flags HintFlags`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub flags: HintFlags,
    /// # C++ Class Fields Info
    /// -   name:`"pad"`
    /// -   type: `hkUint8[3]`
    /// - offset: 5
    /// -  flags: `FLAGS_NONE`
    pub pad: CStyleArray<[u8; 3]>,
}

impl Serialize for HkVertexFormatElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkVertexFormatElementVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkVertexFormatElement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkVertexFormatElementVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkVertexFormatElementVisitor>> for HkVertexFormatElement {
    fn from(_values: Vec<HkVertexFormatElementVisitor>) -> Self {
            let mut data_type = None;
            let mut num_values = None;
            let mut usage = None;
            let mut sub_usage = None;
            let mut flags = None;
            let mut pad = None;


        for _value in _values {
            match _value {
                HkVertexFormatElementVisitor::DataType(m) => data_type = Some(m),
                HkVertexFormatElementVisitor::NumValues(m) => num_values = Some(m),
                HkVertexFormatElementVisitor::Usage(m) => usage = Some(m),
                HkVertexFormatElementVisitor::SubUsage(m) => sub_usage = Some(m),
                HkVertexFormatElementVisitor::Flags(m) => flags = Some(m),
                HkVertexFormatElementVisitor::Pad(m) => pad = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            data_type: data_type.unwrap_or_default().into_inner(),
            num_values: num_values.unwrap_or_default().into_inner(),
            usage: usage.unwrap_or_default().into_inner(),
            sub_usage: sub_usage.unwrap_or_default().into_inner(),
            flags: flags.unwrap_or_default().into_inner(),
            pad: pad.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkVertexFormatElement> for Vec<HkVertexFormatElementVisitor> {
    fn from(data: &HkVertexFormatElement) -> Self {
        vec![
            HkVertexFormatElementVisitor::DataType(data.data_type.clone().into()),
            HkVertexFormatElementVisitor::NumValues(data.num_values.into()),
            HkVertexFormatElementVisitor::Usage(data.usage.clone().into()),
            HkVertexFormatElementVisitor::SubUsage(data.sub_usage.into()),
            HkVertexFormatElementVisitor::Flags(data.flags.clone().into()),
            HkVertexFormatElementVisitor::Pad(data.pad.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkVertexFormatElement {
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
enum HkVertexFormatElementVisitor {
    /// Visitor fields
    #[serde(rename = "dataType")]
    DataType(Primitive<ComponentType>),
    /// Visitor fields
    #[serde(rename = "numValues")]
    NumValues(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "usage")]
    Usage(Primitive<ComponentUsage>),
    /// Visitor fields
    #[serde(rename = "subUsage")]
    SubUsage(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "flags")]
    Flags(Primitive<HintFlags>),
    /// Visitor fields
    #[serde(rename = "pad")]
    Pad(CStyleArray<[u8; 3]>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkVertexFormatElementVisitor, "@name",
    ("dataType" => DataType(Primitive<ComponentType>)),
    ("numValues" => NumValues(Primitive<u8>)),
    ("usage" => Usage(Primitive<ComponentUsage>)),
    ("subUsage" => SubUsage(Primitive<u8>)),
    ("flags" => Flags(Primitive<HintFlags>)),
    ("pad" => Pad(CStyleArray<[u8; 3]>)),
}
