//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleRayCastWheelCollide`
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

/// `hkpVehicleRayCastWheelCollide`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 36
/// -    vtable: true
/// -    parent: `hkpVehicleWheelCollide`/`0x4a50fcb`
/// - signature: `0x41efd9e3`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpVehicleRayCastWheelCollide<'a> {
    /// # C++ Parent class(`hkpVehicleWheelCollide` => parent: `hkReferencedObject`) field Info
    /// -   name:`"alreadyUsed"`
    /// -   type: `hkBool`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub already_used: bool,
    /// # C++ Parent class(`hkpVehicleWheelCollide` => parent: `hkReferencedObject`) field Info
    /// -   name:`"type"`
    /// -   type: `enum unknown`
    /// - offset: 9
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub _type: (),

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
    /// -   name:`"wheelCollisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub wheel_collision_filter_info: u32,
    /// # C++ Class Fields Info
    /// -   name:`"phantom"`
    /// -   type: `struct hkpAabbPhantom*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub phantom: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"rejectRayChassisListener"`
    /// -   type: `struct hkpRejectChassisListener`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub reject_ray_chassis_listener: SingleClass<HkpRejectChassisListener<'a>>,
}

impl Serialize for HkpVehicleRayCastWheelCollide<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpVehicleRayCastWheelCollideVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpVehicleRayCastWheelCollide<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpVehicleRayCastWheelCollideVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpVehicleRayCastWheelCollideVisitor<'a>>> for HkpVehicleRayCastWheelCollide<'a> {
    fn from(_values: Vec<HkpVehicleRayCastWheelCollideVisitor<'a>>) -> Self {
            let mut already_used = None;
            let mut _type = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut wheel_collision_filter_info = None;
            let mut phantom = None;
            let mut reject_ray_chassis_listener = None;


        for _value in _values {
            match _value {
                HkpVehicleRayCastWheelCollideVisitor::AlreadyUsed(m) => already_used = Some(m),
                HkpVehicleRayCastWheelCollideVisitor::Type(m) => _type = Some(m),
                HkpVehicleRayCastWheelCollideVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpVehicleRayCastWheelCollideVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpVehicleRayCastWheelCollideVisitor::WheelCollisionFilterInfo(m) => wheel_collision_filter_info = Some(m),
                HkpVehicleRayCastWheelCollideVisitor::Phantom(m) => phantom = Some(m),
                HkpVehicleRayCastWheelCollideVisitor::RejectRayChassisListener(m) => reject_ray_chassis_listener = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            already_used: already_used.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            wheel_collision_filter_info: wheel_collision_filter_info.unwrap_or_default().into_inner(),
            phantom: phantom.unwrap_or_default().into_inner(),
            reject_ray_chassis_listener: reject_ray_chassis_listener.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpVehicleRayCastWheelCollide<'a>> for Vec<HkpVehicleRayCastWheelCollideVisitor<'a>> {
    fn from(data: &HkpVehicleRayCastWheelCollide<'a>) -> Self {
        vec![
            HkpVehicleRayCastWheelCollideVisitor::AlreadyUsed(data.already_used.into()),
            HkpVehicleRayCastWheelCollideVisitor::Type(data._type.into()),
            HkpVehicleRayCastWheelCollideVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpVehicleRayCastWheelCollideVisitor::ReferenceCount(data.reference_count.into()),
            HkpVehicleRayCastWheelCollideVisitor::WheelCollisionFilterInfo(data.wheel_collision_filter_info.into()),
            HkpVehicleRayCastWheelCollideVisitor::Phantom(data.phantom.clone().into()),
            HkpVehicleRayCastWheelCollideVisitor::RejectRayChassisListener(data.reject_ray_chassis_listener.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpVehicleRayCastWheelCollide<'de> {
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
enum HkpVehicleRayCastWheelCollideVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "alreadyUsed")]
    AlreadyUsed(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "type", skip_serializing)]
    Type(Primitive<()>),

    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "wheelCollisionFilterInfo")]
    WheelCollisionFilterInfo(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "phantom")]
    Phantom(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "rejectRayChassisListener")]
    RejectRayChassisListener(SingleClass<HkpRejectChassisListener<'a>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleRayCastWheelCollideVisitor<'de>, "@name",
    ("alreadyUsed" => AlreadyUsed(Primitive<bool>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("wheelCollisionFilterInfo" => WheelCollisionFilterInfo(Primitive<u32>)),
    ("phantom" => Phantom(Primitive<Cow<'de, str>>)),
    ("rejectRayChassisListener" => RejectRayChassisListener(SingleClass<HkpRejectChassisListener<'de>>)),
}
