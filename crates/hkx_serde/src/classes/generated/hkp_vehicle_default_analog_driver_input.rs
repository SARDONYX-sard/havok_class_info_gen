//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleDefaultAnalogDriverInput`
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

/// `hkpVehicleDefaultAnalogDriverInput`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 24
/// -    vtable: true
/// -    parent: `hkpVehicleDriverInput`/`0xda8c7d7d`
/// - signature: `0x123a5d50`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpVehicleDefaultAnalogDriverInput {
    // C++ Parent class(`hkpVehicleDriverInput` => parent: `hkReferencedObject`) has no fields
    //
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
    /// -   name:`"slopeChangePointX"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub slope_change_point_x: f32,
    /// # C++ Class Fields Info
    /// -   name:`"initialSlope"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub initial_slope: f32,
    /// # C++ Class Fields Info
    /// -   name:`"deadZone"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub dead_zone: f32,
    /// # C++ Class Fields Info
    /// -   name:`"autoReverse"`
    /// -   type: `hkBool`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub auto_reverse: bool,
}

impl Serialize for HkpVehicleDefaultAnalogDriverInput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpVehicleDefaultAnalogDriverInputVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpVehicleDefaultAnalogDriverInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpVehicleDefaultAnalogDriverInputVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpVehicleDefaultAnalogDriverInputVisitor>> for HkpVehicleDefaultAnalogDriverInput {
    fn from(_values: Vec<HkpVehicleDefaultAnalogDriverInputVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut slope_change_point_x = None;
            let mut initial_slope = None;
            let mut dead_zone = None;
            let mut auto_reverse = None;


        for _value in _values {
            match _value {
                HkpVehicleDefaultAnalogDriverInputVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpVehicleDefaultAnalogDriverInputVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpVehicleDefaultAnalogDriverInputVisitor::SlopeChangePointX(m) => slope_change_point_x = Some(m),
                HkpVehicleDefaultAnalogDriverInputVisitor::InitialSlope(m) => initial_slope = Some(m),
                HkpVehicleDefaultAnalogDriverInputVisitor::DeadZone(m) => dead_zone = Some(m),
                HkpVehicleDefaultAnalogDriverInputVisitor::AutoReverse(m) => auto_reverse = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            slope_change_point_x: slope_change_point_x.unwrap_or_default().into_inner(),
            initial_slope: initial_slope.unwrap_or_default().into_inner(),
            dead_zone: dead_zone.unwrap_or_default().into_inner(),
            auto_reverse: auto_reverse.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpVehicleDefaultAnalogDriverInput> for Vec<HkpVehicleDefaultAnalogDriverInputVisitor> {
    fn from(data: &HkpVehicleDefaultAnalogDriverInput) -> Self {
        vec![
            HkpVehicleDefaultAnalogDriverInputVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpVehicleDefaultAnalogDriverInputVisitor::ReferenceCount(data.reference_count.into()),
            HkpVehicleDefaultAnalogDriverInputVisitor::SlopeChangePointX(data.slope_change_point_x.into()),
            HkpVehicleDefaultAnalogDriverInputVisitor::InitialSlope(data.initial_slope.into()),
            HkpVehicleDefaultAnalogDriverInputVisitor::DeadZone(data.dead_zone.into()),
            HkpVehicleDefaultAnalogDriverInputVisitor::AutoReverse(data.auto_reverse.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpVehicleDefaultAnalogDriverInput {
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
enum HkpVehicleDefaultAnalogDriverInputVisitor {
    // C++ Parent class(`hkpVehicleDriverInput` => parent: `hkReferencedObject`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "slopeChangePointX")]
    SlopeChangePointX(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "initialSlope")]
    InitialSlope(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "deadZone")]
    DeadZone(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "autoReverse")]
    AutoReverse(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultAnalogDriverInputVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("slopeChangePointX" => SlopeChangePointX(Primitive<f32>)),
    ("initialSlope" => InitialSlope(Primitive<f32>)),
    ("deadZone" => DeadZone(Primitive<f32>)),
    ("autoReverse" => AutoReverse(Primitive<bool>)),
}
