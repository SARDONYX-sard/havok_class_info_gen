//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpShapeInfo`
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

/// `hkpShapeInfo`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xea7f1d08`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpShapeInfo<'a> {
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
    /// -   name:`"shape"`
    /// -   type: `struct hkpShape*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub shape: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"isHierarchicalCompound"`
    /// -   type: `hkBool`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub is_hierarchical_compound: bool,
    /// # C++ Class Fields Info
    /// -   name:`"hkdShapesCollected"`
    /// -   type: `hkBool`
    /// - offset: 13
    /// -  flags: `FLAGS_NONE`
    pub hkd_shapes_collected: bool,
    /// # C++ Class Fields Info
    /// -   name:`"childShapeNames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub child_shape_names: HkArrayStringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"childTransforms"`
    /// -   type: `hkArray<hkTransform>`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub child_transforms: HkArrayMatrix4<Transform<f32>>,
    /// # C++ Class Fields Info
    /// -   name:`"transform"`
    /// -   type: `hkTransform`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub transform: Transform<f32>,
}

impl Serialize for HkpShapeInfo<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpShapeInfoVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpShapeInfo<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpShapeInfoVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpShapeInfoVisitor<'a>>> for HkpShapeInfo<'a> {
    fn from(_values: Vec<HkpShapeInfoVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut shape = None;
            let mut is_hierarchical_compound = None;
            let mut hkd_shapes_collected = None;
            let mut child_shape_names = None;
            let mut child_transforms = None;
            let mut transform = None;


        for _value in _values {
            match _value {
                HkpShapeInfoVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpShapeInfoVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpShapeInfoVisitor::Shape(m) => shape = Some(m),
                HkpShapeInfoVisitor::IsHierarchicalCompound(m) => is_hierarchical_compound = Some(m),
                HkpShapeInfoVisitor::HkdShapesCollected(m) => hkd_shapes_collected = Some(m),
                HkpShapeInfoVisitor::ChildShapeNames(m) => child_shape_names = Some(m),
                HkpShapeInfoVisitor::ChildTransforms(m) => child_transforms = Some(m),
                HkpShapeInfoVisitor::Transform(m) => transform = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            shape: shape.unwrap_or_default().into_inner(),
            is_hierarchical_compound: is_hierarchical_compound.unwrap_or_default().into_inner(),
            hkd_shapes_collected: hkd_shapes_collected.unwrap_or_default().into_inner(),
            child_shape_names: child_shape_names.unwrap_or_default(),
            child_transforms: child_transforms.unwrap_or_default(),
            transform: transform.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpShapeInfo<'a>> for Vec<HkpShapeInfoVisitor<'a>> {
    fn from(data: &HkpShapeInfo<'a>) -> Self {
        vec![
            HkpShapeInfoVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpShapeInfoVisitor::ReferenceCount(data.reference_count.into()),
            HkpShapeInfoVisitor::Shape(data.shape.clone().into()),
            HkpShapeInfoVisitor::IsHierarchicalCompound(data.is_hierarchical_compound.into()),
            HkpShapeInfoVisitor::HkdShapesCollected(data.hkd_shapes_collected.into()),
            HkpShapeInfoVisitor::ChildShapeNames(data.child_shape_names.clone()),
            HkpShapeInfoVisitor::ChildTransforms(data.child_transforms.clone()),
            HkpShapeInfoVisitor::Transform(data.transform.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpShapeInfo<'de> {
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
enum HkpShapeInfoVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "shape")]
    Shape(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "isHierarchicalCompound")]
    IsHierarchicalCompound(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "hkdShapesCollected")]
    HkdShapesCollected(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "childShapeNames")]
    ChildShapeNames(HkArrayStringPtr<'a>),
    /// Visitor fields
    #[serde(rename = "childTransforms")]
    ChildTransforms(HkArrayMatrix4<Transform<f32>>),
    /// Visitor fields
    #[serde(rename = "transform")]
    Transform(Primitive<Transform<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpShapeInfoVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("shape" => Shape(Primitive<Cow<'de, str>>)),
    ("isHierarchicalCompound" => IsHierarchicalCompound(Primitive<bool>)),
    ("hkdShapesCollected" => HkdShapesCollected(Primitive<bool>)),
    ("childShapeNames" => ChildShapeNames(HkArrayStringPtr<'de>)),
    ("childTransforms" => ChildTransforms(HkArrayMatrix4<Transform<f32>>)),
    ("transform" => Transform(Primitive<Transform<f32>>)),
}
