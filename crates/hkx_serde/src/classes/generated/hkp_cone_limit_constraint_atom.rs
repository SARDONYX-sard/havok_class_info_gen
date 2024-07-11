//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpConeLimitConstraintAtom`
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

/// `hkpConeLimitConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0xf19443c8`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpConeLimitConstraintAtom {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub _type: AtomType,

    /// # C++ Class Fields Info
    /// -   name:`"isEnabled"`
    /// -   type: `hkUint8`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    pub is_enabled: u8,
    /// # C++ Class Fields Info
    /// -   name:`"twistAxisInA"`
    /// -   type: `hkUint8`
    /// - offset: 3
    /// -  flags: `FLAGS_NONE`
    pub twist_axis_in_a: u8,
    /// # C++ Class Fields Info
    /// -   name:`"refAxisInB"`
    /// -   type: `hkUint8`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub ref_axis_in_b: u8,
    /// # C++ Class Fields Info
    /// -   name:`"angleMeasurementMode"`
    /// -   type: `enum MeasurementMode`
    /// - offset: 5
    /// -  flags: `FLAGS_NONE`
    pub angle_measurement_mode: MeasurementMode,
    /// # C++ Class Fields Info
    /// -   name:`"memOffsetToAngleOffset"`
    /// -   type: `hkUint8`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    pub mem_offset_to_angle_offset: u8,
    /// # C++ Class Fields Info
    /// -   name:`"minAngle"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub min_angle: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxAngle"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub max_angle: f32,
    /// # C++ Class Fields Info
    /// -   name:`"angularLimitsTauFactor"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub angular_limits_tau_factor: f32,
}

impl Serialize for HkpConeLimitConstraintAtom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpConeLimitConstraintAtomVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpConeLimitConstraintAtom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpConeLimitConstraintAtomVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpConeLimitConstraintAtomVisitor>> for HkpConeLimitConstraintAtom {
    fn from(_values: Vec<HkpConeLimitConstraintAtomVisitor>) -> Self {
            let mut _type = None;
            let mut is_enabled = None;
            let mut twist_axis_in_a = None;
            let mut ref_axis_in_b = None;
            let mut angle_measurement_mode = None;
            let mut mem_offset_to_angle_offset = None;
            let mut min_angle = None;
            let mut max_angle = None;
            let mut angular_limits_tau_factor = None;


        for _value in _values {
            match _value {
                HkpConeLimitConstraintAtomVisitor::Type(m) => _type = Some(m),
                HkpConeLimitConstraintAtomVisitor::IsEnabled(m) => is_enabled = Some(m),
                HkpConeLimitConstraintAtomVisitor::TwistAxisInA(m) => twist_axis_in_a = Some(m),
                HkpConeLimitConstraintAtomVisitor::RefAxisInB(m) => ref_axis_in_b = Some(m),
                HkpConeLimitConstraintAtomVisitor::AngleMeasurementMode(m) => angle_measurement_mode = Some(m),
                HkpConeLimitConstraintAtomVisitor::MemOffsetToAngleOffset(m) => mem_offset_to_angle_offset = Some(m),
                HkpConeLimitConstraintAtomVisitor::MinAngle(m) => min_angle = Some(m),
                HkpConeLimitConstraintAtomVisitor::MaxAngle(m) => max_angle = Some(m),
                HkpConeLimitConstraintAtomVisitor::AngularLimitsTauFactor(m) => angular_limits_tau_factor = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),
            is_enabled: is_enabled.unwrap_or_default().into_inner(),
            twist_axis_in_a: twist_axis_in_a.unwrap_or_default().into_inner(),
            ref_axis_in_b: ref_axis_in_b.unwrap_or_default().into_inner(),
            angle_measurement_mode: angle_measurement_mode.unwrap_or_default().into_inner(),
            mem_offset_to_angle_offset: mem_offset_to_angle_offset.unwrap_or_default().into_inner(),
            min_angle: min_angle.unwrap_or_default().into_inner(),
            max_angle: max_angle.unwrap_or_default().into_inner(),
            angular_limits_tau_factor: angular_limits_tau_factor.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpConeLimitConstraintAtom> for Vec<HkpConeLimitConstraintAtomVisitor> {
    fn from(data: &HkpConeLimitConstraintAtom) -> Self {
        vec![
            HkpConeLimitConstraintAtomVisitor::Type(data._type.clone().into()),
            HkpConeLimitConstraintAtomVisitor::IsEnabled(data.is_enabled.into()),
            HkpConeLimitConstraintAtomVisitor::TwistAxisInA(data.twist_axis_in_a.into()),
            HkpConeLimitConstraintAtomVisitor::RefAxisInB(data.ref_axis_in_b.into()),
            HkpConeLimitConstraintAtomVisitor::AngleMeasurementMode(data.angle_measurement_mode.clone().into()),
            HkpConeLimitConstraintAtomVisitor::MemOffsetToAngleOffset(data.mem_offset_to_angle_offset.into()),
            HkpConeLimitConstraintAtomVisitor::MinAngle(data.min_angle.into()),
            HkpConeLimitConstraintAtomVisitor::MaxAngle(data.max_angle.into()),
            HkpConeLimitConstraintAtomVisitor::AngularLimitsTauFactor(data.angular_limits_tau_factor.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpConeLimitConstraintAtom {
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
enum HkpConeLimitConstraintAtomVisitor {
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// Visitor fields
    #[serde(rename = "isEnabled")]
    IsEnabled(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "twistAxisInA")]
    TwistAxisInA(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "refAxisInB")]
    RefAxisInB(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "angleMeasurementMode")]
    AngleMeasurementMode(Primitive<MeasurementMode>),
    /// Visitor fields
    #[serde(rename = "memOffsetToAngleOffset")]
    MemOffsetToAngleOffset(Primitive<u8>),
    /// Visitor fields
    #[serde(rename = "minAngle")]
    MinAngle(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxAngle")]
    MaxAngle(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "angularLimitsTauFactor")]
    AngularLimitsTauFactor(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConeLimitConstraintAtomVisitor, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("isEnabled" => IsEnabled(Primitive<u8>)),
    ("twistAxisInA" => TwistAxisInA(Primitive<u8>)),
    ("refAxisInB" => RefAxisInB(Primitive<u8>)),
    ("angleMeasurementMode" => AngleMeasurementMode(Primitive<MeasurementMode>)),
    ("memOffsetToAngleOffset" => MemOffsetToAngleOffset(Primitive<u8>)),
    ("minAngle" => MinAngle(Primitive<f32>)),
    ("maxAngle" => MaxAngle(Primitive<f32>)),
    ("angularLimitsTauFactor" => AngularLimitsTauFactor(Primitive<f32>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum MeasurementMode {
    #[serde(rename = "ZERO_WHEN_VECTORS_ALIGNED")]
    #[default]
    ZeroWhenVectorsAligned = 0,
    #[serde(rename = "ZERO_WHEN_VECTORS_PERPENDICULAR")]
    ZeroWhenVectorsPerpendicular = 1,
}
