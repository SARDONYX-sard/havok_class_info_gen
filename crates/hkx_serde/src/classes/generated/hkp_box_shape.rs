//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpBoxShape`
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

/// `hkpBoxShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: true
/// -    parent: `hkpConvexShape`/`0xf8f74f85`
/// - signature: `0x3444d2d5`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpBoxShape {
    /// # C++ Parent class(`hkpConvexShape` => parent: `hkpSphereRepShape`) field Info
    /// -   name:`"radius"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub radius: f32,

    // C++ Parent class(`hkpSphereRepShape` => parent: `hkpShape`) has no fields
    //
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
    /// -   name:`"halfExtents"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub half_extents: Vector4<f32>,
}

impl Serialize for HkpBoxShape {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpBoxShapeVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpBoxShape {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpBoxShapeVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpBoxShapeVisitor>> for HkpBoxShape {
    fn from(_values: Vec<HkpBoxShapeVisitor>) -> Self {
            let mut radius = None;
            let mut user_data = None;
            let mut _type = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut half_extents = None;


        for _value in _values {
            match _value {
                HkpBoxShapeVisitor::Radius(m) => radius = Some(m),
                HkpBoxShapeVisitor::UserData(m) => user_data = Some(m),
                HkpBoxShapeVisitor::Type(m) => _type = Some(m),
                HkpBoxShapeVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpBoxShapeVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpBoxShapeVisitor::HalfExtents(m) => half_extents = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            radius: radius.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            half_extents: half_extents.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpBoxShape> for Vec<HkpBoxShapeVisitor> {
    fn from(data: &HkpBoxShape) -> Self {
        vec![
            HkpBoxShapeVisitor::Radius(data.radius.into()),
            HkpBoxShapeVisitor::UserData(data.user_data.into()),
            HkpBoxShapeVisitor::Type(data._type.into()),
            HkpBoxShapeVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpBoxShapeVisitor::ReferenceCount(data.reference_count.into()),
            HkpBoxShapeVisitor::HalfExtents(data.half_extents.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpBoxShape {
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
enum HkpBoxShapeVisitor {
    /// Visitor fields
    #[serde(rename = "radius")]
    Radius(Primitive<f32>),

    // C++ Parent class(`hkpSphereRepShape` => parent: `hkpShape`) has no fields
    //
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
    #[serde(rename = "halfExtents")]
    HalfExtents(Primitive<Vector4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpBoxShapeVisitor, "@name",
    ("radius" => Radius(Primitive<f32>)),
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("halfExtents" => HalfExtents(Primitive<Vector4<f32>>)),
}
