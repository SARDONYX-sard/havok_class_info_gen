//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpRackAndPinionConstraintAtom`
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

/// `hkpRackAndPinionConstraintAtom`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 12
/// -    vtable: false
/// -    parent: `hkpConstraintAtom`/`0x59d67ef6`
/// - signature: `0x30cae006`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpRackAndPinionConstraintAtom {
    /// # C++ Parent class(`hkpConstraintAtom` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum AtomType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub _type: AtomType,

    /// # C++ Class Fields Info
    /// -   name:`"pinionRadiusOrScrewPitch"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub pinion_radius_or_screw_pitch: f32,
    /// # C++ Class Fields Info
    /// -   name:`"isScrew"`
    /// -   type: `hkBool`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub is_screw: bool,
    /// # C++ Class Fields Info
    /// -   name:`"memOffsetToInitialAngleOffset"`
    /// -   type: `hkInt8`
    /// - offset: 9
    /// -  flags: `FLAGS_NONE`
    pub mem_offset_to_initial_angle_offset: i8,
    /// # C++ Class Fields Info
    /// -   name:`"memOffsetToPrevAngle"`
    /// -   type: `hkInt8`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    pub mem_offset_to_prev_angle: i8,
    /// # C++ Class Fields Info
    /// -   name:`"memOffsetToRevolutionCounter"`
    /// -   type: `hkInt8`
    /// - offset: 11
    /// -  flags: `FLAGS_NONE`
    pub mem_offset_to_revolution_counter: i8,
}

impl Serialize for HkpRackAndPinionConstraintAtom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpRackAndPinionConstraintAtomVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpRackAndPinionConstraintAtom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpRackAndPinionConstraintAtomVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpRackAndPinionConstraintAtomVisitor>> for HkpRackAndPinionConstraintAtom {
    fn from(_values: Vec<HkpRackAndPinionConstraintAtomVisitor>) -> Self {
            let mut _type = None;
            let mut pinion_radius_or_screw_pitch = None;
            let mut is_screw = None;
            let mut mem_offset_to_initial_angle_offset = None;
            let mut mem_offset_to_prev_angle = None;
            let mut mem_offset_to_revolution_counter = None;


        for _value in _values {
            match _value {
                HkpRackAndPinionConstraintAtomVisitor::Type(m) => _type = Some(m),
                HkpRackAndPinionConstraintAtomVisitor::PinionRadiusOrScrewPitch(m) => pinion_radius_or_screw_pitch = Some(m),
                HkpRackAndPinionConstraintAtomVisitor::IsScrew(m) => is_screw = Some(m),
                HkpRackAndPinionConstraintAtomVisitor::MemOffsetToInitialAngleOffset(m) => mem_offset_to_initial_angle_offset = Some(m),
                HkpRackAndPinionConstraintAtomVisitor::MemOffsetToPrevAngle(m) => mem_offset_to_prev_angle = Some(m),
                HkpRackAndPinionConstraintAtomVisitor::MemOffsetToRevolutionCounter(m) => mem_offset_to_revolution_counter = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),
            pinion_radius_or_screw_pitch: pinion_radius_or_screw_pitch.unwrap_or_default().into_inner(),
            is_screw: is_screw.unwrap_or_default().into_inner(),
            mem_offset_to_initial_angle_offset: mem_offset_to_initial_angle_offset.unwrap_or_default().into_inner(),
            mem_offset_to_prev_angle: mem_offset_to_prev_angle.unwrap_or_default().into_inner(),
            mem_offset_to_revolution_counter: mem_offset_to_revolution_counter.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpRackAndPinionConstraintAtom> for Vec<HkpRackAndPinionConstraintAtomVisitor> {
    fn from(data: &HkpRackAndPinionConstraintAtom) -> Self {
        vec![
            HkpRackAndPinionConstraintAtomVisitor::Type(data._type.clone().into()),
            HkpRackAndPinionConstraintAtomVisitor::PinionRadiusOrScrewPitch(data.pinion_radius_or_screw_pitch.into()),
            HkpRackAndPinionConstraintAtomVisitor::IsScrew(data.is_screw.into()),
            HkpRackAndPinionConstraintAtomVisitor::MemOffsetToInitialAngleOffset(data.mem_offset_to_initial_angle_offset.into()),
            HkpRackAndPinionConstraintAtomVisitor::MemOffsetToPrevAngle(data.mem_offset_to_prev_angle.into()),
            HkpRackAndPinionConstraintAtomVisitor::MemOffsetToRevolutionCounter(data.mem_offset_to_revolution_counter.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpRackAndPinionConstraintAtom {
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
enum HkpRackAndPinionConstraintAtomVisitor {
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<AtomType>),

    /// Visitor fields
    #[serde(rename = "pinionRadiusOrScrewPitch")]
    PinionRadiusOrScrewPitch(Primitive<f32>),
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
    HkpRackAndPinionConstraintAtomVisitor, "@name",
    ("type" => Type(Primitive<AtomType>)),
    ("pinionRadiusOrScrewPitch" => PinionRadiusOrScrewPitch(Primitive<f32>)),
    ("isScrew" => IsScrew(Primitive<bool>)),
    ("memOffsetToInitialAngleOffset" => MemOffsetToInitialAngleOffset(Primitive<i8>)),
    ("memOffsetToPrevAngle" => MemOffsetToPrevAngle(Primitive<i8>)),
    ("memOffsetToRevolutionCounter" => MemOffsetToRevolutionCounter(Primitive<i8>)),
}
