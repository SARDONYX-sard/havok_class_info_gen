//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleDefaultSuspension`
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

/// `hkpVehicleDefaultSuspension`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkpVehicleSuspension`/`0xaf5056fa`
/// - signature: `0x21735a24`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpVehicleDefaultSuspension {
    /// # C++ Parent class(`hkpVehicleSuspension` => parent: `hkReferencedObject`) field Info
    /// -   name:`"wheelParams"`
    /// -   type: `hkArray<struct hkpVehicleSuspensionSuspensionWheelParameters>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub wheel_params: HkArrayClass<HkpVehicleSuspensionSuspensionWheelParameters>,

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
    /// -   name:`"wheelSpringParams"`
    /// -   type: `hkArray<struct hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub wheel_spring_params: HkArrayClass<HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters>,
}

impl Serialize for HkpVehicleDefaultSuspension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpVehicleDefaultSuspensionVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpVehicleDefaultSuspension {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpVehicleDefaultSuspensionVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpVehicleDefaultSuspensionVisitor>> for HkpVehicleDefaultSuspension {
    fn from(_values: Vec<HkpVehicleDefaultSuspensionVisitor>) -> Self {
            let mut wheel_params = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut wheel_spring_params = None;


        for _value in _values {
            match _value {
                HkpVehicleDefaultSuspensionVisitor::WheelParams(m) => wheel_params = Some(m),
                HkpVehicleDefaultSuspensionVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpVehicleDefaultSuspensionVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpVehicleDefaultSuspensionVisitor::WheelSpringParams(m) => wheel_spring_params = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            wheel_params: wheel_params.unwrap_or_default(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            wheel_spring_params: wheel_spring_params.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpVehicleDefaultSuspension> for Vec<HkpVehicleDefaultSuspensionVisitor> {
    fn from(data: &HkpVehicleDefaultSuspension) -> Self {
        vec![
            HkpVehicleDefaultSuspensionVisitor::WheelParams(data.wheel_params.clone()),
            HkpVehicleDefaultSuspensionVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpVehicleDefaultSuspensionVisitor::ReferenceCount(data.reference_count.into()),
            HkpVehicleDefaultSuspensionVisitor::WheelSpringParams(data.wheel_spring_params.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpVehicleDefaultSuspension {
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
enum HkpVehicleDefaultSuspensionVisitor {
    /// Visitor fields
    #[serde(rename = "wheelParams")]
    WheelParams(HkArrayClass<HkpVehicleSuspensionSuspensionWheelParameters>),

    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "wheelSpringParams")]
    WheelSpringParams(HkArrayClass<HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultSuspensionVisitor, "@name",
    ("wheelParams" => WheelParams(HkArrayClass<HkpVehicleSuspensionSuspensionWheelParameters>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("wheelSpringParams" => WheelSpringParams(HkArrayClass<HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters>)),
}
