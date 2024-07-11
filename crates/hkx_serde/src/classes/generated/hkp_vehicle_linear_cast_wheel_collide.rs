//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleLinearCastWheelCollide`
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

/// `hkpVehicleLinearCastWheelCollide`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 52
/// -    vtable: true
/// -    parent: `hkpVehicleWheelCollide`/`0x4a50fcb`
/// - signature: `0xc59399d0`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpVehicleLinearCastWheelCollide<'a> {
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
    /// -   name:`"wheelStates"`
    /// -   type: `hkArray<struct hkpVehicleLinearCastWheelCollideWheelState>`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub wheel_states: HkArrayClass<HkpVehicleLinearCastWheelCollideWheelState<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"rejectChassisListener"`
    /// -   type: `struct hkpRejectChassisListener`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub reject_chassis_listener: SingleClass<HkpRejectChassisListener<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"maxExtraPenetration"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub max_extra_penetration: f32,
    /// # C++ Class Fields Info
    /// -   name:`"startPointTolerance"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub start_point_tolerance: f32,
}

impl Serialize for HkpVehicleLinearCastWheelCollide<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpVehicleLinearCastWheelCollideVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpVehicleLinearCastWheelCollide<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpVehicleLinearCastWheelCollideVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpVehicleLinearCastWheelCollideVisitor<'a>>> for HkpVehicleLinearCastWheelCollide<'a> {
    fn from(_values: Vec<HkpVehicleLinearCastWheelCollideVisitor<'a>>) -> Self {
            let mut already_used = None;
            let mut _type = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut wheel_collision_filter_info = None;
            let mut wheel_states = None;
            let mut reject_chassis_listener = None;
            let mut max_extra_penetration = None;
            let mut start_point_tolerance = None;


        for _value in _values {
            match _value {
                HkpVehicleLinearCastWheelCollideVisitor::AlreadyUsed(m) => already_used = Some(m),
                HkpVehicleLinearCastWheelCollideVisitor::Type(m) => _type = Some(m),
                HkpVehicleLinearCastWheelCollideVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpVehicleLinearCastWheelCollideVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpVehicleLinearCastWheelCollideVisitor::WheelCollisionFilterInfo(m) => wheel_collision_filter_info = Some(m),
                HkpVehicleLinearCastWheelCollideVisitor::WheelStates(m) => wheel_states = Some(m),
                HkpVehicleLinearCastWheelCollideVisitor::RejectChassisListener(m) => reject_chassis_listener = Some(m),
                HkpVehicleLinearCastWheelCollideVisitor::MaxExtraPenetration(m) => max_extra_penetration = Some(m),
                HkpVehicleLinearCastWheelCollideVisitor::StartPointTolerance(m) => start_point_tolerance = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            already_used: already_used.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            wheel_collision_filter_info: wheel_collision_filter_info.unwrap_or_default().into_inner(),
            wheel_states: wheel_states.unwrap_or_default(),
            reject_chassis_listener: reject_chassis_listener.unwrap_or_default(),
            max_extra_penetration: max_extra_penetration.unwrap_or_default().into_inner(),
            start_point_tolerance: start_point_tolerance.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpVehicleLinearCastWheelCollide<'a>> for Vec<HkpVehicleLinearCastWheelCollideVisitor<'a>> {
    fn from(data: &HkpVehicleLinearCastWheelCollide<'a>) -> Self {
        vec![
            HkpVehicleLinearCastWheelCollideVisitor::AlreadyUsed(data.already_used.into()),
            HkpVehicleLinearCastWheelCollideVisitor::Type(data._type.into()),
            HkpVehicleLinearCastWheelCollideVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpVehicleLinearCastWheelCollideVisitor::ReferenceCount(data.reference_count.into()),
            HkpVehicleLinearCastWheelCollideVisitor::WheelCollisionFilterInfo(data.wheel_collision_filter_info.into()),
            HkpVehicleLinearCastWheelCollideVisitor::WheelStates(data.wheel_states.clone()),
            HkpVehicleLinearCastWheelCollideVisitor::RejectChassisListener(data.reject_chassis_listener.clone()),
            HkpVehicleLinearCastWheelCollideVisitor::MaxExtraPenetration(data.max_extra_penetration.into()),
            HkpVehicleLinearCastWheelCollideVisitor::StartPointTolerance(data.start_point_tolerance.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpVehicleLinearCastWheelCollide<'de> {
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
enum HkpVehicleLinearCastWheelCollideVisitor<'a> {
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
    #[serde(rename = "wheelStates")]
    WheelStates(HkArrayClass<HkpVehicleLinearCastWheelCollideWheelState<'a>>),
    /// Visitor fields
    #[serde(rename = "rejectChassisListener")]
    RejectChassisListener(SingleClass<HkpRejectChassisListener<'a>>),
    /// Visitor fields
    #[serde(rename = "maxExtraPenetration")]
    MaxExtraPenetration(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "startPointTolerance")]
    StartPointTolerance(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleLinearCastWheelCollideVisitor<'de>, "@name",
    ("alreadyUsed" => AlreadyUsed(Primitive<bool>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("wheelCollisionFilterInfo" => WheelCollisionFilterInfo(Primitive<u32>)),
    ("wheelStates" => WheelStates(HkArrayClass<HkpVehicleLinearCastWheelCollideWheelState<'de>>)),
    ("rejectChassisListener" => RejectChassisListener(SingleClass<HkpRejectChassisListener<'de>>)),
    ("maxExtraPenetration" => MaxExtraPenetration(Primitive<f32>)),
    ("startPointTolerance" => StartPointTolerance(Primitive<f32>)),
}
