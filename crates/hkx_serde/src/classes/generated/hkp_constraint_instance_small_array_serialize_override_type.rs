//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpConstraintInstanceSmallArraySerializeOverrideType`
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

/// `hkpConstraintInstanceSmallArraySerializeOverrideType`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 8
/// -    vtable: false
/// - signature: `0xee3c2aec`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpConstraintInstanceSmallArraySerializeOverrideType<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `void*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub data: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"size"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub size: u16,
    /// # C++ Class Fields Info
    /// -   name:`"capacityAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    pub capacity_and_flags: u16,
}

impl Serialize for HkpConstraintInstanceSmallArraySerializeOverrideType<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpConstraintInstanceSmallArraySerializeOverrideTypeVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpConstraintInstanceSmallArraySerializeOverrideType<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpConstraintInstanceSmallArraySerializeOverrideTypeVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpConstraintInstanceSmallArraySerializeOverrideTypeVisitor<'a>>> for HkpConstraintInstanceSmallArraySerializeOverrideType<'a> {
    fn from(_values: Vec<HkpConstraintInstanceSmallArraySerializeOverrideTypeVisitor<'a>>) -> Self {
            let mut data = None;
            let mut size = None;
            let mut capacity_and_flags = None;


        for _value in _values {
            match _value {
                HkpConstraintInstanceSmallArraySerializeOverrideTypeVisitor::Data(m) => data = Some(m),
                HkpConstraintInstanceSmallArraySerializeOverrideTypeVisitor::Size(m) => size = Some(m),
                HkpConstraintInstanceSmallArraySerializeOverrideTypeVisitor::CapacityAndFlags(m) => capacity_and_flags = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            data: data.unwrap_or_default().into_inner(),
            size: size.unwrap_or_default().into_inner(),
            capacity_and_flags: capacity_and_flags.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpConstraintInstanceSmallArraySerializeOverrideType<'a>> for Vec<HkpConstraintInstanceSmallArraySerializeOverrideTypeVisitor<'a>> {
    fn from(data: &HkpConstraintInstanceSmallArraySerializeOverrideType<'a>) -> Self {
        vec![
            HkpConstraintInstanceSmallArraySerializeOverrideTypeVisitor::Data(data.data.clone().into()),
            HkpConstraintInstanceSmallArraySerializeOverrideTypeVisitor::Size(data.size.into()),
            HkpConstraintInstanceSmallArraySerializeOverrideTypeVisitor::CapacityAndFlags(data.capacity_and_flags.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpConstraintInstanceSmallArraySerializeOverrideType<'de> {
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
enum HkpConstraintInstanceSmallArraySerializeOverrideTypeVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "data", skip_serializing)]
    Data(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "size")]
    Size(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "capacityAndFlags")]
    CapacityAndFlags(Primitive<u16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConstraintInstanceSmallArraySerializeOverrideTypeVisitor<'de>, "@name",
    ("data" => Data(Primitive<Cow<'de, str>>)),
    ("size" => Size(Primitive<u16>)),
    ("capacityAndFlags" => CapacityAndFlags(Primitive<u16>)),
}
