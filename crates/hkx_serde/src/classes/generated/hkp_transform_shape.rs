//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpTransformShape`
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

/// `hkpTransformShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkpShape`/`0x666490a1`
/// - signature: `0x787ef513`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpTransformShape<'a> {
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
    /// -   name:`"childShape"`
    /// -   type: `struct hkpSingleShapeContainer`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub child_shape: SingleClass<HkpSingleShapeContainer<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"childShapeSize"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub child_shape_size: i32,
    /// # C++ Class Fields Info
    /// -   name:`"rotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub rotation: Quaternion<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"transform"`
    /// -   type: `hkTransform`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub transform: Transform<f32>,
}

impl Serialize for HkpTransformShape<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpTransformShapeVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpTransformShape<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpTransformShapeVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpTransformShapeVisitor<'a>>> for HkpTransformShape<'a> {
    fn from(_values: Vec<HkpTransformShapeVisitor<'a>>) -> Self {
            let mut user_data = None;
            let mut _type = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut child_shape = None;
            let mut child_shape_size = None;
            let mut rotation = None;
            let mut transform = None;


        for _value in _values {
            match _value {
                HkpTransformShapeVisitor::UserData(m) => user_data = Some(m),
                HkpTransformShapeVisitor::Type(m) => _type = Some(m),
                HkpTransformShapeVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpTransformShapeVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpTransformShapeVisitor::ChildShape(m) => child_shape = Some(m),
                HkpTransformShapeVisitor::ChildShapeSize(m) => child_shape_size = Some(m),
                HkpTransformShapeVisitor::Rotation(m) => rotation = Some(m),
                HkpTransformShapeVisitor::Transform(m) => transform = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            user_data: user_data.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            child_shape: child_shape.unwrap_or_default(),
            child_shape_size: child_shape_size.unwrap_or_default().into_inner(),
            rotation: rotation.unwrap_or_default().into_inner(),
            transform: transform.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpTransformShape<'a>> for Vec<HkpTransformShapeVisitor<'a>> {
    fn from(data: &HkpTransformShape<'a>) -> Self {
        vec![
            HkpTransformShapeVisitor::UserData(data.user_data.into()),
            HkpTransformShapeVisitor::Type(data._type.into()),
            HkpTransformShapeVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpTransformShapeVisitor::ReferenceCount(data.reference_count.into()),
            HkpTransformShapeVisitor::ChildShape(data.child_shape.clone()),
            HkpTransformShapeVisitor::ChildShapeSize(data.child_shape_size.into()),
            HkpTransformShapeVisitor::Rotation(data.rotation.clone().into()),
            HkpTransformShapeVisitor::Transform(data.transform.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpTransformShape<'de> {
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
enum HkpTransformShapeVisitor<'a> {
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
    #[serde(rename = "childShape")]
    ChildShape(SingleClass<HkpSingleShapeContainer<'a>>),
    /// Visitor fields
    #[serde(rename = "childShapeSize", skip_serializing)]
    ChildShapeSize(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "rotation")]
    Rotation(Primitive<Quaternion<f32>>),
    /// Visitor fields
    #[serde(rename = "transform")]
    Transform(Primitive<Transform<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpTransformShapeVisitor<'de>, "@name",
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("childShape" => ChildShape(SingleClass<HkpSingleShapeContainer<'de>>)),
    ("childShapeSize" => ChildShapeSize(Primitive<i32>)),
    ("rotation" => Rotation(Primitive<Quaternion<f32>>)),
    ("transform" => Transform(Primitive<Transform<f32>>)),
}
