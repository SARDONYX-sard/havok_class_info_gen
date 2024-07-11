//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpConvexVerticesShape`
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

/// `hkpConvexVerticesShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: true
/// -    parent: `hkpConvexShape`/`0xf8f74f85`
/// - signature: `0x28726ad8`
/// -   version: 3
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpConvexVerticesShape<'a> {
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
    /// -   name:`"rotatedVertices"`
    /// -   type: `hkArray<struct hkpConvexVerticesShapeFourVectors>`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub rotated_vertices: HkArrayClass<HkpConvexVerticesShapeFourVectors>,
    /// # C++ Class Fields Info
    /// -   name:`"numVertices"`
    /// -   type: `hkInt32`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    pub num_vertices: i32,
    /// # C++ Class Fields Info
    /// -   name:`"externalObject"`
    /// -   type: `void*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub external_object: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"getFaceNormals"`
    /// -   type: `void*`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub get_face_normals: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"planeEquations"`
    /// -   type: `hkArray<hkVector4>`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    pub plane_equations: HkArrayVector<Vector4<f32>>,
    /// # C++ Class Fields Info
    /// -   name:`"connectivity"`
    /// -   type: `struct hkpConvexVerticesConnectivity*`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    pub connectivity: Cow<'a, str>,
}

impl Serialize for HkpConvexVerticesShape<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpConvexVerticesShapeVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpConvexVerticesShape<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpConvexVerticesShapeVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpConvexVerticesShapeVisitor<'a>>> for HkpConvexVerticesShape<'a> {
    fn from(_values: Vec<HkpConvexVerticesShapeVisitor<'a>>) -> Self {
            let mut radius = None;
            let mut user_data = None;
            let mut _type = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut aabb_half_extents = None;
            let mut aabb_center = None;
            let mut rotated_vertices = None;
            let mut num_vertices = None;
            let mut external_object = None;
            let mut get_face_normals = None;
            let mut plane_equations = None;
            let mut connectivity = None;


        for _value in _values {
            match _value {
                HkpConvexVerticesShapeVisitor::Radius(m) => radius = Some(m),
                HkpConvexVerticesShapeVisitor::UserData(m) => user_data = Some(m),
                HkpConvexVerticesShapeVisitor::Type(m) => _type = Some(m),
                HkpConvexVerticesShapeVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpConvexVerticesShapeVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpConvexVerticesShapeVisitor::AabbHalfExtents(m) => aabb_half_extents = Some(m),
                HkpConvexVerticesShapeVisitor::AabbCenter(m) => aabb_center = Some(m),
                HkpConvexVerticesShapeVisitor::RotatedVertices(m) => rotated_vertices = Some(m),
                HkpConvexVerticesShapeVisitor::NumVertices(m) => num_vertices = Some(m),
                HkpConvexVerticesShapeVisitor::ExternalObject(m) => external_object = Some(m),
                HkpConvexVerticesShapeVisitor::GetFaceNormals(m) => get_face_normals = Some(m),
                HkpConvexVerticesShapeVisitor::PlaneEquations(m) => plane_equations = Some(m),
                HkpConvexVerticesShapeVisitor::Connectivity(m) => connectivity = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            radius: radius.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            aabb_half_extents: aabb_half_extents.unwrap_or_default().into_inner(),
            aabb_center: aabb_center.unwrap_or_default().into_inner(),
            rotated_vertices: rotated_vertices.unwrap_or_default(),
            num_vertices: num_vertices.unwrap_or_default().into_inner(),
            external_object: external_object.unwrap_or_default().into_inner(),
            get_face_normals: get_face_normals.unwrap_or_default().into_inner(),
            plane_equations: plane_equations.unwrap_or_default(),
            connectivity: connectivity.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpConvexVerticesShape<'a>> for Vec<HkpConvexVerticesShapeVisitor<'a>> {
    fn from(data: &HkpConvexVerticesShape<'a>) -> Self {
        vec![
            HkpConvexVerticesShapeVisitor::Radius(data.radius.into()),
            HkpConvexVerticesShapeVisitor::UserData(data.user_data.into()),
            HkpConvexVerticesShapeVisitor::Type(data._type.into()),
            HkpConvexVerticesShapeVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpConvexVerticesShapeVisitor::ReferenceCount(data.reference_count.into()),
            HkpConvexVerticesShapeVisitor::AabbHalfExtents(data.aabb_half_extents.into()),
            HkpConvexVerticesShapeVisitor::AabbCenter(data.aabb_center.into()),
            HkpConvexVerticesShapeVisitor::RotatedVertices(data.rotated_vertices.clone()),
            HkpConvexVerticesShapeVisitor::NumVertices(data.num_vertices.into()),
            HkpConvexVerticesShapeVisitor::ExternalObject(data.external_object.clone().into()),
            HkpConvexVerticesShapeVisitor::GetFaceNormals(data.get_face_normals.clone().into()),
            HkpConvexVerticesShapeVisitor::PlaneEquations(data.plane_equations.clone()),
            HkpConvexVerticesShapeVisitor::Connectivity(data.connectivity.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpConvexVerticesShape<'de> {
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
enum HkpConvexVerticesShapeVisitor<'a> {
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
    #[serde(rename = "aabbHalfExtents")]
    AabbHalfExtents(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "aabbCenter")]
    AabbCenter(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "rotatedVertices")]
    RotatedVertices(HkArrayClass<HkpConvexVerticesShapeFourVectors>),
    /// Visitor fields
    #[serde(rename = "numVertices")]
    NumVertices(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "externalObject", skip_serializing)]
    ExternalObject(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "getFaceNormals", skip_serializing)]
    GetFaceNormals(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "planeEquations")]
    PlaneEquations(HkArrayVector<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "connectivity")]
    Connectivity(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConvexVerticesShapeVisitor<'de>, "@name",
    ("radius" => Radius(Primitive<f32>)),
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("aabbHalfExtents" => AabbHalfExtents(Primitive<Vector4<f32>>)),
    ("aabbCenter" => AabbCenter(Primitive<Vector4<f32>>)),
    ("rotatedVertices" => RotatedVertices(HkArrayClass<HkpConvexVerticesShapeFourVectors>)),
    ("numVertices" => NumVertices(Primitive<i32>)),
    ("externalObject" => ExternalObject(Primitive<Cow<'de, str>>)),
    ("getFaceNormals" => GetFaceNormals(Primitive<Cow<'de, str>>)),
    ("planeEquations" => PlaneEquations(HkArrayVector<Vector4<f32>>)),
    ("connectivity" => Connectivity(Primitive<Cow<'de, str>>)),
}
