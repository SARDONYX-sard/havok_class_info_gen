//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpTyremarksWheel`
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

/// `hkpTyremarksWheel`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 28
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x1eaef041`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpTyremarksWheel {
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
    /// -   name:`"currentPosition"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub current_position: i32,
    /// # C++ Class Fields Info
    /// -   name:`"numPoints"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub num_points: i32,
    /// # C++ Class Fields Info
    /// -   name:`"tyremarkPoints"`
    /// -   type: `hkArray<struct hkpTyremarkPoint>`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub tyremark_points: HkArrayClass<HkpTyremarkPoint>,
}

impl Serialize for HkpTyremarksWheel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpTyremarksWheelVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpTyremarksWheel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpTyremarksWheelVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpTyremarksWheelVisitor>> for HkpTyremarksWheel {
    fn from(_values: Vec<HkpTyremarksWheelVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut current_position = None;
            let mut num_points = None;
            let mut tyremark_points = None;


        for _value in _values {
            match _value {
                HkpTyremarksWheelVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpTyremarksWheelVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpTyremarksWheelVisitor::CurrentPosition(m) => current_position = Some(m),
                HkpTyremarksWheelVisitor::NumPoints(m) => num_points = Some(m),
                HkpTyremarksWheelVisitor::TyremarkPoints(m) => tyremark_points = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            current_position: current_position.unwrap_or_default().into_inner(),
            num_points: num_points.unwrap_or_default().into_inner(),
            tyremark_points: tyremark_points.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpTyremarksWheel> for Vec<HkpTyremarksWheelVisitor> {
    fn from(data: &HkpTyremarksWheel) -> Self {
        vec![
            HkpTyremarksWheelVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpTyremarksWheelVisitor::ReferenceCount(data.reference_count.into()),
            HkpTyremarksWheelVisitor::CurrentPosition(data.current_position.into()),
            HkpTyremarksWheelVisitor::NumPoints(data.num_points.into()),
            HkpTyremarksWheelVisitor::TyremarkPoints(data.tyremark_points.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpTyremarksWheel {
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
enum HkpTyremarksWheelVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "currentPosition")]
    CurrentPosition(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "numPoints")]
    NumPoints(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "tyremarkPoints")]
    TyremarkPoints(HkArrayClass<HkpTyremarkPoint>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpTyremarksWheelVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("currentPosition" => CurrentPosition(Primitive<i32>)),
    ("numPoints" => NumPoints(Primitive<i32>)),
    ("tyremarkPoints" => TyremarkPoints(HkArrayClass<HkpTyremarkPoint>)),
}
