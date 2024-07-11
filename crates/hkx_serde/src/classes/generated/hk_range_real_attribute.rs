//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkRangeRealAttribute`
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

/// `hkRangeRealAttribute`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x949db24f`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkRangeRealAttribute {
    /// # C++ Class Fields Info
    /// -   name:`"absmin"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub absmin: f32,
    /// # C++ Class Fields Info
    /// -   name:`"absmax"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub absmax: f32,
    /// # C++ Class Fields Info
    /// -   name:`"softmin"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub softmin: f32,
    /// # C++ Class Fields Info
    /// -   name:`"softmax"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub softmax: f32,
}

impl Serialize for HkRangeRealAttribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkRangeRealAttributeVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkRangeRealAttribute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkRangeRealAttributeVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkRangeRealAttributeVisitor>> for HkRangeRealAttribute {
    fn from(_values: Vec<HkRangeRealAttributeVisitor>) -> Self {
            let mut absmin = None;
            let mut absmax = None;
            let mut softmin = None;
            let mut softmax = None;


        for _value in _values {
            match _value {
                HkRangeRealAttributeVisitor::Absmin(m) => absmin = Some(m),
                HkRangeRealAttributeVisitor::Absmax(m) => absmax = Some(m),
                HkRangeRealAttributeVisitor::Softmin(m) => softmin = Some(m),
                HkRangeRealAttributeVisitor::Softmax(m) => softmax = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            absmin: absmin.unwrap_or_default().into_inner(),
            absmax: absmax.unwrap_or_default().into_inner(),
            softmin: softmin.unwrap_or_default().into_inner(),
            softmax: softmax.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkRangeRealAttribute> for Vec<HkRangeRealAttributeVisitor> {
    fn from(data: &HkRangeRealAttribute) -> Self {
        vec![
            HkRangeRealAttributeVisitor::Absmin(data.absmin.into()),
            HkRangeRealAttributeVisitor::Absmax(data.absmax.into()),
            HkRangeRealAttributeVisitor::Softmin(data.softmin.into()),
            HkRangeRealAttributeVisitor::Softmax(data.softmax.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkRangeRealAttribute {
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
enum HkRangeRealAttributeVisitor {
    /// Visitor fields
    #[serde(rename = "absmin")]
    Absmin(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "absmax")]
    Absmax(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "softmin")]
    Softmin(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "softmax")]
    Softmax(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkRangeRealAttributeVisitor, "@name",
    ("absmin" => Absmin(Primitive<f32>)),
    ("absmax" => Absmax(Primitive<f32>)),
    ("softmin" => Softmin(Primitive<f32>)),
    ("softmax" => Softmax(Primitive<f32>)),
}
