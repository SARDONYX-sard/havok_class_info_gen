//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpMultiRayShape`
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

/// `hkpMultiRayShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkpShape`/`0x666490a1`
/// - signature: `0xea2e7ec9`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpMultiRayShape {
    /// # C++ Parent class(`hkpShape` => parent: `hkReferencedObject`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub user_data: usize,
    /// # C++ Parent class(`hkpShape` => parent: `hkReferencedObject`) field Info
    /// -   name:`"type"`
    /// -   type: `enum unknown`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub _type: (),

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
    /// -   name:`"rays"`
    /// -   type: `hkArray<struct hkpMultiRayShapeRay>`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub rays: HkArrayClass<HkpMultiRayShapeRay>,
    /// # C++ Class Fields Info
    /// -   name:`"rayPenetrationDistance"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub ray_penetration_distance: f32,
}

impl Serialize for HkpMultiRayShape {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpMultiRayShapeVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpMultiRayShape {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpMultiRayShapeVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpMultiRayShapeVisitor>> for HkpMultiRayShape {
    fn from(_values: Vec<HkpMultiRayShapeVisitor>) -> Self {
            let mut user_data = None;
            let mut _type = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut rays = None;
            let mut ray_penetration_distance = None;


        for _value in _values {
            match _value {
                HkpMultiRayShapeVisitor::UserData(m) => user_data = Some(m),
                HkpMultiRayShapeVisitor::Type(m) => _type = Some(m),
                HkpMultiRayShapeVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpMultiRayShapeVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpMultiRayShapeVisitor::Rays(m) => rays = Some(m),
                HkpMultiRayShapeVisitor::RayPenetrationDistance(m) => ray_penetration_distance = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            user_data: user_data.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            rays: rays.unwrap_or_default(),
            ray_penetration_distance: ray_penetration_distance.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpMultiRayShape> for Vec<HkpMultiRayShapeVisitor> {
    fn from(data: &HkpMultiRayShape) -> Self {
        vec![
            HkpMultiRayShapeVisitor::UserData(data.user_data.into()),
            HkpMultiRayShapeVisitor::Type(data._type.into()),
            HkpMultiRayShapeVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpMultiRayShapeVisitor::ReferenceCount(data.reference_count.into()),
            HkpMultiRayShapeVisitor::Rays(data.rays.clone()),
            HkpMultiRayShapeVisitor::RayPenetrationDistance(data.ray_penetration_distance.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpMultiRayShape {
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
enum HkpMultiRayShapeVisitor {
    /// Visitor fields
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),
    /// Visitor fields
    #[serde(rename = "type", skip_serializing)]
    Type(Primitive<()>),

    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "rays")]
    Rays(HkArrayClass<HkpMultiRayShapeRay>),
    /// Visitor fields
    #[serde(rename = "rayPenetrationDistance")]
    RayPenetrationDistance(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMultiRayShapeVisitor, "@name",
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("rays" => Rays(HkArrayClass<HkpMultiRayShapeRay>)),
    ("rayPenetrationDistance" => RayPenetrationDistance(Primitive<f32>)),
}
