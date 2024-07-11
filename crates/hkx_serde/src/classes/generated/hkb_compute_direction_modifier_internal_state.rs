//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbComputeDirectionModifierInternalState`
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

/// `hkbComputeDirectionModifierInternalState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x6ac054d7`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbComputeDirectionModifierInternalState {
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
    /// -   name:`"pointOut"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub point_out: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"groundAngleOut"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub ground_angle_out: f32,
    /// # C++ Class Fields Info
    /// -   name:`"upAngleOut"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub up_angle_out: f32,
    /// # C++ Class Fields Info
    /// -   name:`"computedOutput"`
    /// -   type: `hkBool`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub computed_output: bool,
}

impl Serialize for HkbComputeDirectionModifierInternalState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbComputeDirectionModifierInternalStateVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbComputeDirectionModifierInternalState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbComputeDirectionModifierInternalStateVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbComputeDirectionModifierInternalStateVisitor>> for HkbComputeDirectionModifierInternalState {
    fn from(_values: Vec<HkbComputeDirectionModifierInternalStateVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut point_out = None;
            let mut ground_angle_out = None;
            let mut up_angle_out = None;
            let mut computed_output = None;


        for _value in _values {
            match _value {
                HkbComputeDirectionModifierInternalStateVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbComputeDirectionModifierInternalStateVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbComputeDirectionModifierInternalStateVisitor::PointOut(m) => point_out = Some(m),
                HkbComputeDirectionModifierInternalStateVisitor::GroundAngleOut(m) => ground_angle_out = Some(m),
                HkbComputeDirectionModifierInternalStateVisitor::UpAngleOut(m) => up_angle_out = Some(m),
                HkbComputeDirectionModifierInternalStateVisitor::ComputedOutput(m) => computed_output = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            point_out: point_out.unwrap_or_default().into_inner(),
            ground_angle_out: ground_angle_out.unwrap_or_default().into_inner(),
            up_angle_out: up_angle_out.unwrap_or_default().into_inner(),
            computed_output: computed_output.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbComputeDirectionModifierInternalState> for Vec<HkbComputeDirectionModifierInternalStateVisitor> {
    fn from(data: &HkbComputeDirectionModifierInternalState) -> Self {
        vec![
            HkbComputeDirectionModifierInternalStateVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbComputeDirectionModifierInternalStateVisitor::ReferenceCount(data.reference_count.into()),
            HkbComputeDirectionModifierInternalStateVisitor::PointOut(data.point_out.into()),
            HkbComputeDirectionModifierInternalStateVisitor::GroundAngleOut(data.ground_angle_out.into()),
            HkbComputeDirectionModifierInternalStateVisitor::UpAngleOut(data.up_angle_out.into()),
            HkbComputeDirectionModifierInternalStateVisitor::ComputedOutput(data.computed_output.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbComputeDirectionModifierInternalState {
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
enum HkbComputeDirectionModifierInternalStateVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "pointOut")]
    PointOut(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "groundAngleOut")]
    GroundAngleOut(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "upAngleOut")]
    UpAngleOut(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "computedOutput")]
    ComputedOutput(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbComputeDirectionModifierInternalStateVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("pointOut" => PointOut(Primitive<Vector4<f32>>)),
    ("groundAngleOut" => GroundAngleOut(Primitive<f32>)),
    ("upAngleOut" => UpAngleOut(Primitive<f32>)),
    ("computedOutput" => ComputedOutput(Primitive<bool>)),
}
