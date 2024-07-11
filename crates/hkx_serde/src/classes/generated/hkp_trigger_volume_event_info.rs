//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpTriggerVolumeEventInfo`
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

/// `hkpTriggerVolumeEventInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0xeb60f431`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpTriggerVolumeEventInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"sortValue"`
    /// -   type: `hkUint64`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub sort_value: u64,
    /// # C++ Class Fields Info
    /// -   name:`"body"`
    /// -   type: `struct hkpRigidBody*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub body: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"operation"`
    /// -   type: `enum Operation`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub operation: Operation,
}

impl Serialize for HkpTriggerVolumeEventInfo<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpTriggerVolumeEventInfoVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpTriggerVolumeEventInfo<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpTriggerVolumeEventInfoVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpTriggerVolumeEventInfoVisitor<'a>>> for HkpTriggerVolumeEventInfo<'a> {
    fn from(_values: Vec<HkpTriggerVolumeEventInfoVisitor<'a>>) -> Self {
            let mut sort_value = None;
            let mut body = None;
            let mut operation = None;


        for _value in _values {
            match _value {
                HkpTriggerVolumeEventInfoVisitor::SortValue(m) => sort_value = Some(m),
                HkpTriggerVolumeEventInfoVisitor::Body(m) => body = Some(m),
                HkpTriggerVolumeEventInfoVisitor::Operation(m) => operation = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            sort_value: sort_value.unwrap_or_default().into_inner(),
            body: body.unwrap_or_default().into_inner(),
            operation: operation.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpTriggerVolumeEventInfo<'a>> for Vec<HkpTriggerVolumeEventInfoVisitor<'a>> {
    fn from(data: &HkpTriggerVolumeEventInfo<'a>) -> Self {
        vec![
            HkpTriggerVolumeEventInfoVisitor::SortValue(data.sort_value.into()),
            HkpTriggerVolumeEventInfoVisitor::Body(data.body.clone().into()),
            HkpTriggerVolumeEventInfoVisitor::Operation(data.operation.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpTriggerVolumeEventInfo<'de> {
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
enum HkpTriggerVolumeEventInfoVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "sortValue")]
    SortValue(Primitive<u64>),
    /// Visitor fields
    #[serde(rename = "body")]
    Body(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "operation")]
    Operation(Primitive<Operation>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpTriggerVolumeEventInfoVisitor<'de>, "@name",
    ("sortValue" => SortValue(Primitive<u64>)),
    ("body" => Body(Primitive<Cow<'de, str>>)),
    ("operation" => Operation(Primitive<Operation>)),
}
