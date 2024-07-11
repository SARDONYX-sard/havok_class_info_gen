//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpConvexTransformShape`
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

/// `hkpConvexTransformShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: true
/// -    parent: `hkpConvexTransformShapeBase`/`0xfbd72f9`
/// - signature: `0xae3e5017`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpConvexTransformShape<'a> {
    /// # C++ Parent class(`hkpConvexTransformShapeBase` => parent: `hkpConvexShape`) field Info
    /// -   name:`"childShape"`
    /// -   type: `struct hkpSingleShapeContainer`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub child_shape: SingleClass<HkpSingleShapeContainer<'a>>,
    /// # C++ Parent class(`hkpConvexTransformShapeBase` => parent: `hkpConvexShape`) field Info
    /// -   name:`"childShapeSize"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub child_shape_size: i32,

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
    /// -   name:`"transform"`
    /// -   type: `hkTransform`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub transform: Transform<f32>,
}

impl Serialize for HkpConvexTransformShape<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpConvexTransformShapeVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpConvexTransformShape<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpConvexTransformShapeVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpConvexTransformShapeVisitor<'a>>> for HkpConvexTransformShape<'a> {
    fn from(_values: Vec<HkpConvexTransformShapeVisitor<'a>>) -> Self {
            let mut child_shape = None;
            let mut child_shape_size = None;
            let mut radius = None;
            let mut user_data = None;
            let mut _type = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut transform = None;


        for _value in _values {
            match _value {
                HkpConvexTransformShapeVisitor::ChildShape(m) => child_shape = Some(m),
                HkpConvexTransformShapeVisitor::ChildShapeSize(m) => child_shape_size = Some(m),
                HkpConvexTransformShapeVisitor::Radius(m) => radius = Some(m),
                HkpConvexTransformShapeVisitor::UserData(m) => user_data = Some(m),
                HkpConvexTransformShapeVisitor::Type(m) => _type = Some(m),
                HkpConvexTransformShapeVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpConvexTransformShapeVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpConvexTransformShapeVisitor::Transform(m) => transform = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            child_shape: child_shape.unwrap_or_default(),
            child_shape_size: child_shape_size.unwrap_or_default().into_inner(),
            radius: radius.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            transform: transform.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpConvexTransformShape<'a>> for Vec<HkpConvexTransformShapeVisitor<'a>> {
    fn from(data: &HkpConvexTransformShape<'a>) -> Self {
        vec![
            HkpConvexTransformShapeVisitor::ChildShape(data.child_shape.clone()),
            HkpConvexTransformShapeVisitor::ChildShapeSize(data.child_shape_size.into()),
            HkpConvexTransformShapeVisitor::Radius(data.radius.into()),
            HkpConvexTransformShapeVisitor::UserData(data.user_data.into()),
            HkpConvexTransformShapeVisitor::Type(data._type.into()),
            HkpConvexTransformShapeVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpConvexTransformShapeVisitor::ReferenceCount(data.reference_count.into()),
            HkpConvexTransformShapeVisitor::Transform(data.transform.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpConvexTransformShape<'de> {
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
enum HkpConvexTransformShapeVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "childShape")]
    ChildShape(SingleClass<HkpSingleShapeContainer<'a>>),
    /// Visitor fields
    #[serde(rename = "childShapeSize", skip_serializing)]
    ChildShapeSize(Primitive<i32>),

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
    #[serde(rename = "transform")]
    Transform(Primitive<Transform<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConvexTransformShapeVisitor<'de>, "@name",
    ("childShape" => ChildShape(SingleClass<HkpSingleShapeContainer<'de>>)),
    ("childShapeSize" => ChildShapeSize(Primitive<i32>)),
    ("radius" => Radius(Primitive<f32>)),
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("transform" => Transform(Primitive<Transform<f32>>)),
}
