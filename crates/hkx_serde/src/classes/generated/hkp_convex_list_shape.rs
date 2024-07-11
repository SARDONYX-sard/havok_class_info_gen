//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpConvexListShape`
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

/// `hkpConvexListShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: true
/// -    parent: `hkpConvexShape`/`0xf8f74f85`
/// - signature: `0x450b26e8`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpConvexListShape<'a> {
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
    /// -   name:`"minDistanceToUseConvexHullForGetClosestPoints"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub min_distance_to_use_convex_hull_for_get_closest_points: f32,
    /// # C++ Class Fields Info
    /// -   name:`"aabbHalfExtents"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub aabb_half_extents: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"aabbCenter"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub aabb_center: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"useCachedAabb"`
    /// -   type: `hkBool`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub use_cached_aabb: bool,
    /// # C++ Class Fields Info
    /// -   name:`"childShapes"`
    /// -   type: `hkArray<hkpConvexShape*>`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub child_shapes: HkArrayRef<Cow<'a, str>>,
}

impl Serialize for HkpConvexListShape<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpConvexListShapeVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpConvexListShape<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpConvexListShapeVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpConvexListShapeVisitor<'a>>> for HkpConvexListShape<'a> {
    fn from(_values: Vec<HkpConvexListShapeVisitor<'a>>) -> Self {
            let mut radius = None;
            let mut user_data = None;
            let mut _type = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut min_distance_to_use_convex_hull_for_get_closest_points = None;
            let mut aabb_half_extents = None;
            let mut aabb_center = None;
            let mut use_cached_aabb = None;
            let mut child_shapes = None;


        for _value in _values {
            match _value {
                HkpConvexListShapeVisitor::Radius(m) => radius = Some(m),
                HkpConvexListShapeVisitor::UserData(m) => user_data = Some(m),
                HkpConvexListShapeVisitor::Type(m) => _type = Some(m),
                HkpConvexListShapeVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpConvexListShapeVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpConvexListShapeVisitor::MinDistanceToUseConvexHullForGetClosestPoints(m) => min_distance_to_use_convex_hull_for_get_closest_points = Some(m),
                HkpConvexListShapeVisitor::AabbHalfExtents(m) => aabb_half_extents = Some(m),
                HkpConvexListShapeVisitor::AabbCenter(m) => aabb_center = Some(m),
                HkpConvexListShapeVisitor::UseCachedAabb(m) => use_cached_aabb = Some(m),
                HkpConvexListShapeVisitor::ChildShapes(m) => child_shapes = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            radius: radius.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            min_distance_to_use_convex_hull_for_get_closest_points: min_distance_to_use_convex_hull_for_get_closest_points.unwrap_or_default().into_inner(),
            aabb_half_extents: aabb_half_extents.unwrap_or_default().into_inner(),
            aabb_center: aabb_center.unwrap_or_default().into_inner(),
            use_cached_aabb: use_cached_aabb.unwrap_or_default().into_inner(),
            child_shapes: child_shapes.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpConvexListShape<'a>> for Vec<HkpConvexListShapeVisitor<'a>> {
    fn from(data: &HkpConvexListShape<'a>) -> Self {
        vec![
            HkpConvexListShapeVisitor::Radius(data.radius.into()),
            HkpConvexListShapeVisitor::UserData(data.user_data.into()),
            HkpConvexListShapeVisitor::Type(data._type.into()),
            HkpConvexListShapeVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpConvexListShapeVisitor::ReferenceCount(data.reference_count.into()),
            HkpConvexListShapeVisitor::MinDistanceToUseConvexHullForGetClosestPoints(data.min_distance_to_use_convex_hull_for_get_closest_points.into()),
            HkpConvexListShapeVisitor::AabbHalfExtents(data.aabb_half_extents.into()),
            HkpConvexListShapeVisitor::AabbCenter(data.aabb_center.into()),
            HkpConvexListShapeVisitor::UseCachedAabb(data.use_cached_aabb.into()),
            HkpConvexListShapeVisitor::ChildShapes(data.child_shapes.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpConvexListShape<'de> {
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
enum HkpConvexListShapeVisitor<'a> {
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
    #[serde(rename = "minDistanceToUseConvexHullForGetClosestPoints")]
    MinDistanceToUseConvexHullForGetClosestPoints(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "aabbHalfExtents")]
    AabbHalfExtents(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "aabbCenter")]
    AabbCenter(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "useCachedAabb")]
    UseCachedAabb(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "childShapes")]
    ChildShapes(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConvexListShapeVisitor<'de>, "@name",
    ("radius" => Radius(Primitive<f32>)),
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("minDistanceToUseConvexHullForGetClosestPoints" => MinDistanceToUseConvexHullForGetClosestPoints(Primitive<f32>)),
    ("aabbHalfExtents" => AabbHalfExtents(Primitive<Vector4<f32>>)),
    ("aabbCenter" => AabbCenter(Primitive<Vector4<f32>>)),
    ("useCachedAabb" => UseCachedAabb(Primitive<bool>)),
    ("childShapes" => ChildShapes(HkArrayRef<Cow<'de, str>>)),
}
