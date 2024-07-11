//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters`
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

/// `hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// - signature: `0x7be5bed1`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters {
    /// # C++ Class Fields Info
    /// -   name:`"strength"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub strength: f32,
    /// # C++ Class Fields Info
    /// -   name:`"dampingCompression"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub damping_compression: f32,
    /// # C++ Class Fields Info
    /// -   name:`"dampingRelaxation"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub damping_relaxation: f32,
}

impl Serialize for HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpVehicleDefaultSuspensionWheelSpringSuspensionParametersVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpVehicleDefaultSuspensionWheelSpringSuspensionParametersVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpVehicleDefaultSuspensionWheelSpringSuspensionParametersVisitor>> for HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters {
    fn from(_values: Vec<HkpVehicleDefaultSuspensionWheelSpringSuspensionParametersVisitor>) -> Self {
            let mut strength = None;
            let mut damping_compression = None;
            let mut damping_relaxation = None;


        for _value in _values {
            match _value {
                HkpVehicleDefaultSuspensionWheelSpringSuspensionParametersVisitor::Strength(m) => strength = Some(m),
                HkpVehicleDefaultSuspensionWheelSpringSuspensionParametersVisitor::DampingCompression(m) => damping_compression = Some(m),
                HkpVehicleDefaultSuspensionWheelSpringSuspensionParametersVisitor::DampingRelaxation(m) => damping_relaxation = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            strength: strength.unwrap_or_default().into_inner(),
            damping_compression: damping_compression.unwrap_or_default().into_inner(),
            damping_relaxation: damping_relaxation.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters> for Vec<HkpVehicleDefaultSuspensionWheelSpringSuspensionParametersVisitor> {
    fn from(data: &HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters) -> Self {
        vec![
            HkpVehicleDefaultSuspensionWheelSpringSuspensionParametersVisitor::Strength(data.strength.into()),
            HkpVehicleDefaultSuspensionWheelSpringSuspensionParametersVisitor::DampingCompression(data.damping_compression.into()),
            HkpVehicleDefaultSuspensionWheelSpringSuspensionParametersVisitor::DampingRelaxation(data.damping_relaxation.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpVehicleDefaultSuspensionWheelSpringSuspensionParameters {
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
enum HkpVehicleDefaultSuspensionWheelSpringSuspensionParametersVisitor {
    /// Visitor fields
    #[serde(rename = "strength")]
    Strength(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "dampingCompression")]
    DampingCompression(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "dampingRelaxation")]
    DampingRelaxation(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultSuspensionWheelSpringSuspensionParametersVisitor, "@name",
    ("strength" => Strength(Primitive<f32>)),
    ("dampingCompression" => DampingCompression(Primitive<f32>)),
    ("dampingRelaxation" => DampingRelaxation(Primitive<f32>)),
}
