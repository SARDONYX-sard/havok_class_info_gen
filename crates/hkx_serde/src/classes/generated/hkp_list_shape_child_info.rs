//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpListShapeChildInfo`
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

/// `hkpListShapeChildInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x80df0f90`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpListShapeChildInfo<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"shape"`
    /// -   type: `struct hkpShape*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|ALIGN16`
    pub shape: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub collision_filter_info: u32,
    /// # C++ Class Fields Info
    /// -   name:`"shapeSize"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub shape_size: i32,
    /// # C++ Class Fields Info
    /// -   name:`"numChildShapes"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub num_child_shapes: i32,
}

impl Serialize for HkpListShapeChildInfo<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpListShapeChildInfoVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpListShapeChildInfo<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpListShapeChildInfoVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpListShapeChildInfoVisitor<'a>>> for HkpListShapeChildInfo<'a> {
    fn from(_values: Vec<HkpListShapeChildInfoVisitor<'a>>) -> Self {
            let mut shape = None;
            let mut collision_filter_info = None;
            let mut shape_size = None;
            let mut num_child_shapes = None;


        for _value in _values {
            match _value {
                HkpListShapeChildInfoVisitor::Shape(m) => shape = Some(m),
                HkpListShapeChildInfoVisitor::CollisionFilterInfo(m) => collision_filter_info = Some(m),
                HkpListShapeChildInfoVisitor::ShapeSize(m) => shape_size = Some(m),
                HkpListShapeChildInfoVisitor::NumChildShapes(m) => num_child_shapes = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            shape: shape.unwrap_or_default().into_inner(),
            collision_filter_info: collision_filter_info.unwrap_or_default().into_inner(),
            shape_size: shape_size.unwrap_or_default().into_inner(),
            num_child_shapes: num_child_shapes.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpListShapeChildInfo<'a>> for Vec<HkpListShapeChildInfoVisitor<'a>> {
    fn from(data: &HkpListShapeChildInfo<'a>) -> Self {
        vec![
            HkpListShapeChildInfoVisitor::Shape(data.shape.clone().into()),
            HkpListShapeChildInfoVisitor::CollisionFilterInfo(data.collision_filter_info.into()),
            HkpListShapeChildInfoVisitor::ShapeSize(data.shape_size.into()),
            HkpListShapeChildInfoVisitor::NumChildShapes(data.num_child_shapes.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpListShapeChildInfo<'de> {
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
enum HkpListShapeChildInfoVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "shape")]
    Shape(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "shapeSize", skip_serializing)]
    ShapeSize(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "numChildShapes", skip_serializing)]
    NumChildShapes(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpListShapeChildInfoVisitor<'de>, "@name",
    ("shape" => Shape(Primitive<Cow<'de, str>>)),
    ("collisionFilterInfo" => CollisionFilterInfo(Primitive<u32>)),
    ("shapeSize" => ShapeSize(Primitive<i32>)),
    ("numChildShapes" => NumChildShapes(Primitive<i32>)),
}
