//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbDampingModifierInternalState`
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

/// `hkbDampingModifierInternalState`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x508d3b36`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbDampingModifierInternalState {
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
    /// -   name:`"dampedVector"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub damped_vector: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"vecErrorSum"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub vec_error_sum: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"vecPreviousError"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub vec_previous_error: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"dampedValue"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub damped_value: f32,
    /// # C++ Class Fields Info
    /// -   name:`"errorSum"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub error_sum: f32,
    /// # C++ Class Fields Info
    /// -   name:`"previousError"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    pub previous_error: f32,
}

impl Serialize for HkbDampingModifierInternalState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbDampingModifierInternalStateVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbDampingModifierInternalState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbDampingModifierInternalStateVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbDampingModifierInternalStateVisitor>> for HkbDampingModifierInternalState {
    fn from(_values: Vec<HkbDampingModifierInternalStateVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut damped_vector = None;
            let mut vec_error_sum = None;
            let mut vec_previous_error = None;
            let mut damped_value = None;
            let mut error_sum = None;
            let mut previous_error = None;


        for _value in _values {
            match _value {
                HkbDampingModifierInternalStateVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbDampingModifierInternalStateVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbDampingModifierInternalStateVisitor::DampedVector(m) => damped_vector = Some(m),
                HkbDampingModifierInternalStateVisitor::VecErrorSum(m) => vec_error_sum = Some(m),
                HkbDampingModifierInternalStateVisitor::VecPreviousError(m) => vec_previous_error = Some(m),
                HkbDampingModifierInternalStateVisitor::DampedValue(m) => damped_value = Some(m),
                HkbDampingModifierInternalStateVisitor::ErrorSum(m) => error_sum = Some(m),
                HkbDampingModifierInternalStateVisitor::PreviousError(m) => previous_error = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            damped_vector: damped_vector.unwrap_or_default().into_inner(),
            vec_error_sum: vec_error_sum.unwrap_or_default().into_inner(),
            vec_previous_error: vec_previous_error.unwrap_or_default().into_inner(),
            damped_value: damped_value.unwrap_or_default().into_inner(),
            error_sum: error_sum.unwrap_or_default().into_inner(),
            previous_error: previous_error.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbDampingModifierInternalState> for Vec<HkbDampingModifierInternalStateVisitor> {
    fn from(data: &HkbDampingModifierInternalState) -> Self {
        vec![
            HkbDampingModifierInternalStateVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbDampingModifierInternalStateVisitor::ReferenceCount(data.reference_count.into()),
            HkbDampingModifierInternalStateVisitor::DampedVector(data.damped_vector.into()),
            HkbDampingModifierInternalStateVisitor::VecErrorSum(data.vec_error_sum.into()),
            HkbDampingModifierInternalStateVisitor::VecPreviousError(data.vec_previous_error.into()),
            HkbDampingModifierInternalStateVisitor::DampedValue(data.damped_value.into()),
            HkbDampingModifierInternalStateVisitor::ErrorSum(data.error_sum.into()),
            HkbDampingModifierInternalStateVisitor::PreviousError(data.previous_error.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbDampingModifierInternalState {
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
enum HkbDampingModifierInternalStateVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "dampedVector")]
    DampedVector(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "vecErrorSum")]
    VecErrorSum(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "vecPreviousError")]
    VecPreviousError(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "dampedValue")]
    DampedValue(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "errorSum")]
    ErrorSum(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "previousError")]
    PreviousError(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbDampingModifierInternalStateVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("dampedVector" => DampedVector(Primitive<Vector4<f32>>)),
    ("vecErrorSum" => VecErrorSum(Primitive<Vector4<f32>>)),
    ("vecPreviousError" => VecPreviousError(Primitive<Vector4<f32>>)),
    ("dampedValue" => DampedValue(Primitive<f32>)),
    ("errorSum" => ErrorSum(Primitive<f32>)),
    ("previousError" => PreviousError(Primitive<f32>)),
}
