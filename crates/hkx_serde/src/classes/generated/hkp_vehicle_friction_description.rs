//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleFrictionDescription`
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

/// `hkpVehicleFrictionDescription`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 208
/// -    vtable: false
/// - signature: `0x1034549a`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpVehicleFrictionDescription {
    /// # C++ Class Fields Info
    /// -   name:`"wheelDistance"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub wheel_distance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"chassisMassInv"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub chassis_mass_inv: f32,
    /// # C++ Class Fields Info
    /// -   name:`"axleDescr"`
    /// -   type: `struct hkpVehicleFrictionDescriptionAxisDescription[2]`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub axle_descr: CStyleArrayClass<HkpVehicleFrictionDescriptionAxisDescription, 2>,
}

impl Serialize for HkpVehicleFrictionDescription {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpVehicleFrictionDescriptionVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpVehicleFrictionDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpVehicleFrictionDescriptionVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpVehicleFrictionDescriptionVisitor>> for HkpVehicleFrictionDescription {
    fn from(_values: Vec<HkpVehicleFrictionDescriptionVisitor>) -> Self {
            let mut wheel_distance = None;
            let mut chassis_mass_inv = None;
            let mut axle_descr = None;


        for _value in _values {
            match _value {
                HkpVehicleFrictionDescriptionVisitor::WheelDistance(m) => wheel_distance = Some(m),
                HkpVehicleFrictionDescriptionVisitor::ChassisMassInv(m) => chassis_mass_inv = Some(m),
                HkpVehicleFrictionDescriptionVisitor::AxleDescr(m) => axle_descr = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            wheel_distance: wheel_distance.unwrap_or_default().into_inner(),
            chassis_mass_inv: chassis_mass_inv.unwrap_or_default().into_inner(),
            axle_descr: axle_descr.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpVehicleFrictionDescription> for Vec<HkpVehicleFrictionDescriptionVisitor> {
    fn from(data: &HkpVehicleFrictionDescription) -> Self {
        vec![
            HkpVehicleFrictionDescriptionVisitor::WheelDistance(data.wheel_distance.into()),
            HkpVehicleFrictionDescriptionVisitor::ChassisMassInv(data.chassis_mass_inv.into()),
            HkpVehicleFrictionDescriptionVisitor::AxleDescr(data.axle_descr.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpVehicleFrictionDescription {
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
enum HkpVehicleFrictionDescriptionVisitor {
    /// Visitor fields
    #[serde(rename = "wheelDistance")]
    WheelDistance(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "chassisMassInv")]
    ChassisMassInv(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "axleDescr")]
    AxleDescr(CStyleArrayClass<HkpVehicleFrictionDescriptionAxisDescription, 2>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleFrictionDescriptionVisitor, "@name",
    ("wheelDistance" => WheelDistance(Primitive<f32>)),
    ("chassisMassInv" => ChassisMassInv(Primitive<f32>)),
    ("axleDescr" => AxleDescr(CStyleArrayClass<HkpVehicleFrictionDescriptionAxisDescription, 2>)),
}
