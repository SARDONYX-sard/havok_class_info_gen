//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCogWheelConstraintAtom`
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

/// `hkpCogWheelConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0xf2b1f399`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpCogWheelConstraintAtom {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub _type: AtomType,

    /// # C++ Class Fields Info
    /// -   name:`"cogWheelRadiusA"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub cog_wheel_radius_a: f32,
    /// # C++ Class Fields Info
    /// -   name:`"cogWheelRadiusB"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub cog_wheel_radius_b: f32,
    /// # C++ Class Fields Info
    /// -   name:`"isScrew"`
    /// -   type: `hkBool`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub is_screw: bool,
    /// # C++ Class Fields Info
    /// -   name:`"memOffsetToInitialAngleOffset"`
    /// -   type: `hkInt8`
    /// - offset: 13
    /// -  flags: `FLAGS_NONE`
    pub mem_offset_to_initial_angle_offset: i8,
    /// # C++ Class Fields Info
    /// -   name:`"memOffsetToPrevAngle"`
    /// -   type: `hkInt8`
    /// - offset: 14
    /// -  flags: `FLAGS_NONE`
    pub mem_offset_to_prev_angle: i8,
    /// # C++ Class Fields Info
    /// -   name:`"memOffsetToRevolutionCounter"`
    /// -   type: `hkInt8`
    /// - offset: 15
    /// -  flags: `FLAGS_NONE`
    pub mem_offset_to_revolution_counter: i8,
}

impl Serialize for HkpCogWheelConstraintAtom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpCogWheelConstraintAtomVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpCogWheelConstraintAtom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpCogWheelConstraintAtomVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpCogWheelConstraintAtomVisitor>> for HkpCogWheelConstraintAtom {
    fn from(_values: Vec<HkpCogWheelConstraintAtomVisitor>) -> Self {
            let mut _type = None;
            let mut cog_wheel_radius_a = None;
            let mut cog_wheel_radius_b = None;
            let mut is_screw = None;
            let mut mem_offset_to_initial_angle_offset = None;
            let mut mem_offset_to_prev_angle = None;
            let mut mem_offset_to_revolution_counter = None;


        for _value in _values {
            match _value {
                HkpCogWheelConstraintAtomVisitor::Type(m) => _type = Some(m),
                HkpCogWheelConstraintAtomVisitor::CogWheelRadiusA(m) => cog_wheel_radius_a = Some(m),
                HkpCogWheelConstraintAtomVisitor::CogWheelRadiusB(m) => cog_wheel_radius_b = Some(m),
                HkpCogWheelConstraintAtomVisitor::IsScrew(m) => is_screw = Some(m),
                HkpCogWheelConstraintAtomVisitor::MemOffsetToInitialAngleOffset(m) => mem_offset_to_initial_angle_offset = Some(m),
                HkpCogWheelConstraintAtomVisitor::MemOffsetToPrevAngle(m) => mem_offset_to_prev_angle = Some(m),
                HkpCogWheelConstraintAtomVisitor::MemOffsetToRevolutionCounter(m) => mem_offset_to_revolution_counter = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),
            cog_wheel_radius_a: cog_wheel_radius_a.unwrap_or_default().into_inner(),
            cog_wheel_radius_b: cog_wheel_radius_b.unwrap_or_default().into_inner(),
            is_screw: is_screw.unwrap_or_default().into_inner(),
            mem_offset_to_initial_angle_offset: mem_offset_to_initial_angle_offset.unwrap_or_default().into_inner(),
            mem_offset_to_prev_angle: mem_offset_to_prev_angle.unwrap_or_default().into_inner(),
            mem_offset_to_revolution_counter: mem_offset_to_revolution_counter.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpCogWheelConstraintAtom> for Vec<HkpCogWheelConstraintAtomVisitor> {
    fn from(data: &HkpCogWheelConstraintAtom) -> Self {
        vec![
            HkpCogWheelConstraintAtomVisitor::Type(data._type.clone().into()),
            HkpCogWheelConstraintAtomVisitor::CogWheelRadiusA(data.cog_wheel_radius_a.into()),
            HkpCogWheelConstraintAtomVisitor::CogWheelRadiusB(data.cog_wheel_radius_b.into()),
            HkpCogWheelConstraintAtomVisitor::IsScrew(data.is_screw.into()),
            HkpCogWheelConstraintAtomVisitor::MemOffsetToInitialAngleOffset(data.mem_offset_to_initial_angle_offset.into()),
            HkpCogWheelConstraintAtomVisitor::MemOffsetToPrevAngle(data.mem_offset_to_prev_angle.into()),
            HkpCogWheelConstraintAtomVisitor::MemOffsetToRevolutionCounter(data.mem_offset_to_revolution_counter.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpCogWheelConstraintAtom {
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
enum HkpCogWheelConstraintAtomVisitor {
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// Visitor fields
    #[serde(rename = "cogWheelRadiusA")]
    CogWheelRadiusA(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "cogWheelRadiusB")]
    CogWheelRadiusB(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "isScrew")]
    IsScrew(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "memOffsetToInitialAngleOffset")]
    MemOffsetToInitialAngleOffset(Primitive<i8>),
    /// Visitor fields
    #[serde(rename = "memOffsetToPrevAngle")]
    MemOffsetToPrevAngle(Primitive<i8>),
    /// Visitor fields
    #[serde(rename = "memOffsetToRevolutionCounter")]
    MemOffsetToRevolutionCounter(Primitive<i8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCogWheelConstraintAtomVisitor, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("cogWheelRadiusA" => CogWheelRadiusA(Primitive<f32>)),
    ("cogWheelRadiusB" => CogWheelRadiusB(Primitive<f32>)),
    ("isScrew" => IsScrew(Primitive<bool>)),
    ("memOffsetToInitialAngleOffset" => MemOffsetToInitialAngleOffset(Primitive<i8>)),
    ("memOffsetToPrevAngle" => MemOffsetToPrevAngle(Primitive<i8>)),
    ("memOffsetToRevolutionCounter" => MemOffsetToRevolutionCounter(Primitive<i8>)),
}
