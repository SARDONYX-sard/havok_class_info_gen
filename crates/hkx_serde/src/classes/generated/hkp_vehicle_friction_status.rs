//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleFrictionStatus`
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

/// `hkpVehicleFrictionStatus`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 72
/// -    vtable: false
/// - signature: `0x1c076a84`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpVehicleFrictionStatus {
    /// # C++ Class Fields Info
    /// -   name:`"axis"`
    /// -   type: `struct hkpVehicleFrictionStatusAxisStatus[2]`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub axis: CStyleArrayClass<HkpVehicleFrictionStatusAxisStatus, 2>,
}

impl Serialize for HkpVehicleFrictionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpVehicleFrictionStatusVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpVehicleFrictionStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpVehicleFrictionStatusVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpVehicleFrictionStatusVisitor>> for HkpVehicleFrictionStatus {
    fn from(_values: Vec<HkpVehicleFrictionStatusVisitor>) -> Self {
            let mut axis = None;


        for _value in _values {
            match _value {
                HkpVehicleFrictionStatusVisitor::Axis(m) => axis = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            axis: axis.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpVehicleFrictionStatus> for Vec<HkpVehicleFrictionStatusVisitor> {
    fn from(data: &HkpVehicleFrictionStatus) -> Self {
        vec![
            HkpVehicleFrictionStatusVisitor::Axis(data.axis.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpVehicleFrictionStatus {
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
enum HkpVehicleFrictionStatusVisitor {
    /// Visitor fields
    #[serde(rename = "axis")]
    Axis(CStyleArrayClass<HkpVehicleFrictionStatusAxisStatus, 2>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleFrictionStatusVisitor, "@name",
    ("axis" => Axis(CStyleArrayClass<HkpVehicleFrictionStatusAxisStatus, 2>)),
}
