//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleWheelCollide`
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

/// `hkpVehicleWheelCollide`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x4a50fcb`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpVehicleWheelCollide {
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub mem_size_and_flags: u16,
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub reference_count: i16,

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// # C++ Class Fields Info
    /// -   name:`"alreadyUsed"`
    /// -   type: `hkBool`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub already_used: bool,
    /// # C++ Class Fields Info
    /// -   name:`"type"`
    /// -   type: `enum unknown`
    /// - offset: 9
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub _type: (),
}

impl Serialize for HkpVehicleWheelCollide {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpVehicleWheelCollideVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpVehicleWheelCollide {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpVehicleWheelCollideVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpVehicleWheelCollideVisitor>> for HkpVehicleWheelCollide {
    fn from(_values: Vec<HkpVehicleWheelCollideVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut already_used = None;
            let mut _type = None;


        for _value in _values {
            match _value {
                HkpVehicleWheelCollideVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpVehicleWheelCollideVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpVehicleWheelCollideVisitor::AlreadyUsed(m) => already_used = Some(m),
                HkpVehicleWheelCollideVisitor::Type(m) => _type = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            already_used: already_used.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpVehicleWheelCollide> for Vec<HkpVehicleWheelCollideVisitor> {
    fn from(data: &HkpVehicleWheelCollide) -> Self {
        vec![
            HkpVehicleWheelCollideVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpVehicleWheelCollideVisitor::ReferenceCount(data.reference_count.into()),
            HkpVehicleWheelCollideVisitor::AlreadyUsed(data.already_used.into()),
            HkpVehicleWheelCollideVisitor::Type(data._type.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpVehicleWheelCollide {
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
enum HkpVehicleWheelCollideVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "alreadyUsed")]
    AlreadyUsed(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "type", skip_serializing)]
    Type(Primitive<()>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleWheelCollideVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("alreadyUsed" => AlreadyUsed(Primitive<bool>)),
    ("type" => Type(Primitive<()>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum WheelCollideType {
    #[serde(rename = "INVALID_WHEEL_COLLIDE")]
    #[default]
    InvalidWheelCollide = 0,
    #[serde(rename = "RAY_CAST_WHEEL_COLLIDE")]
    RayCastWheelCollide = 1,
    #[serde(rename = "LINEAR_CAST_WHEEL_COLLIDE")]
    LinearCastWheelCollide = 2,
    #[serde(rename = "USER_WHEEL_COLLIDE1")]
    UserWheelCollide1 = 3,
    #[serde(rename = "USER_WHEEL_COLLIDE2")]
    UserWheelCollide2 = 4,
    #[serde(rename = "USER_WHEEL_COLLIDE3")]
    UserWheelCollide3 = 5,
    #[serde(rename = "USER_WHEEL_COLLIDE4")]
    UserWheelCollide4 = 6,
    #[serde(rename = "USER_WHEEL_COLLIDE5")]
    UserWheelCollide5 = 7,
}
