//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpVehicleDefaultAerodynamics`
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

/// `hkpVehicleDefaultAerodynamics`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkpVehicleAerodynamics`/`0xda8c7d7d`
/// - signature: `0x42fc5bbd`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpVehicleDefaultAerodynamics {
    // C++ Parent class(`hkpVehicleAerodynamics` => parent: `hkReferencedObject`) has no fields
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
    /// -   name:`"airDensity"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub air_density: f32,
    /// # C++ Class Fields Info
    /// -   name:`"frontalArea"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub frontal_area: f32,
    /// # C++ Class Fields Info
    /// -   name:`"dragCoefficient"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub drag_coefficient: f32,
    /// # C++ Class Fields Info
    /// -   name:`"liftCoefficient"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub lift_coefficient: f32,
    /// # C++ Class Fields Info
    /// -   name:`"extraGravityws"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub extra_gravityws: Vector4<f32>,
}

impl Serialize for HkpVehicleDefaultAerodynamics {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpVehicleDefaultAerodynamicsVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpVehicleDefaultAerodynamics {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpVehicleDefaultAerodynamicsVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpVehicleDefaultAerodynamicsVisitor>> for HkpVehicleDefaultAerodynamics {
    fn from(_values: Vec<HkpVehicleDefaultAerodynamicsVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut air_density = None;
            let mut frontal_area = None;
            let mut drag_coefficient = None;
            let mut lift_coefficient = None;
            let mut extra_gravityws = None;


        for _value in _values {
            match _value {
                HkpVehicleDefaultAerodynamicsVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpVehicleDefaultAerodynamicsVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpVehicleDefaultAerodynamicsVisitor::AirDensity(m) => air_density = Some(m),
                HkpVehicleDefaultAerodynamicsVisitor::FrontalArea(m) => frontal_area = Some(m),
                HkpVehicleDefaultAerodynamicsVisitor::DragCoefficient(m) => drag_coefficient = Some(m),
                HkpVehicleDefaultAerodynamicsVisitor::LiftCoefficient(m) => lift_coefficient = Some(m),
                HkpVehicleDefaultAerodynamicsVisitor::ExtraGravityws(m) => extra_gravityws = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            air_density: air_density.unwrap_or_default().into_inner(),
            frontal_area: frontal_area.unwrap_or_default().into_inner(),
            drag_coefficient: drag_coefficient.unwrap_or_default().into_inner(),
            lift_coefficient: lift_coefficient.unwrap_or_default().into_inner(),
            extra_gravityws: extra_gravityws.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpVehicleDefaultAerodynamics> for Vec<HkpVehicleDefaultAerodynamicsVisitor> {
    fn from(data: &HkpVehicleDefaultAerodynamics) -> Self {
        vec![
            HkpVehicleDefaultAerodynamicsVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpVehicleDefaultAerodynamicsVisitor::ReferenceCount(data.reference_count.into()),
            HkpVehicleDefaultAerodynamicsVisitor::AirDensity(data.air_density.into()),
            HkpVehicleDefaultAerodynamicsVisitor::FrontalArea(data.frontal_area.into()),
            HkpVehicleDefaultAerodynamicsVisitor::DragCoefficient(data.drag_coefficient.into()),
            HkpVehicleDefaultAerodynamicsVisitor::LiftCoefficient(data.lift_coefficient.into()),
            HkpVehicleDefaultAerodynamicsVisitor::ExtraGravityws(data.extra_gravityws.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpVehicleDefaultAerodynamics {
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
enum HkpVehicleDefaultAerodynamicsVisitor {
    // C++ Parent class(`hkpVehicleAerodynamics` => parent: `hkReferencedObject`) has no fields
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
    #[serde(rename = "airDensity")]
    AirDensity(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "frontalArea")]
    FrontalArea(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "dragCoefficient")]
    DragCoefficient(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "liftCoefficient")]
    LiftCoefficient(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "extraGravityws")]
    ExtraGravityws(Primitive<Vector4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpVehicleDefaultAerodynamicsVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("airDensity" => AirDensity(Primitive<f32>)),
    ("frontalArea" => FrontalArea(Primitive<f32>)),
    ("dragCoefficient" => DragCoefficient(Primitive<f32>)),
    ("liftCoefficient" => LiftCoefficient(Primitive<f32>)),
    ("extraGravityws" => ExtraGravityws(Primitive<Vector4<f32>>)),
}
