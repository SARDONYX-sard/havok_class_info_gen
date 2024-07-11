//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpExtendedMeshShapeTrianglesSubpart`
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

/// `hkpExtendedMeshShapeTrianglesSubpart`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 112
/// -    vtable: false
/// -    parent: `hkpExtendedMeshShapeSubpart`/`0xf4608207`
/// - signature: `0x44c32df6`
/// -   version: 3
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpExtendedMeshShapeTrianglesSubpart<'a> {
    /// # C++ Parent class(`hkpExtendedMeshShapeSubpart` => parent: `None`) field Info
    /// -   name:`"type"`
    /// -   type: `enum SubpartType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub _type: SubpartType,
    /// # C++ Parent class(`hkpExtendedMeshShapeSubpart` => parent: `None`) field Info
    /// -   name:`"materialIndexStridingType"`
    /// -   type: `enum MaterialIndexStridingType`
    /// - offset: 1
    /// -  flags: `FLAGS_NONE`
    pub material_index_striding_type: MaterialIndexStridingType,
    /// # C++ Parent class(`hkpExtendedMeshShapeSubpart` => parent: `None`) field Info
    /// -   name:`"materialStriding"`
    /// -   type: `hkInt16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub material_striding: i16,
    /// # C++ Parent class(`hkpExtendedMeshShapeSubpart` => parent: `None`) field Info
    /// -   name:`"materialIndexBase"`
    /// -   type: `void*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub material_index_base: Cow<'a, str>,
    /// # C++ Parent class(`hkpExtendedMeshShapeSubpart` => parent: `None`) field Info
    /// -   name:`"materialIndexStriding"`
    /// -   type: `hkUint16`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub material_index_striding: u16,
    /// # C++ Parent class(`hkpExtendedMeshShapeSubpart` => parent: `None`) field Info
    /// -   name:`"numMaterials"`
    /// -   type: `hkUint16`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    pub num_materials: u16,
    /// # C++ Parent class(`hkpExtendedMeshShapeSubpart` => parent: `None`) field Info
    /// -   name:`"materialBase"`
    /// -   type: `void*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub material_base: Cow<'a, str>,
    /// # C++ Parent class(`hkpExtendedMeshShapeSubpart` => parent: `None`) field Info
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub user_data: usize,

    /// # C++ Class Fields Info
    /// -   name:`"numTriangleShapes"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub num_triangle_shapes: i32,
    /// # C++ Class Fields Info
    /// -   name:`"vertexBase"`
    /// -   type: `void*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub vertex_base: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"numVertices"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub num_vertices: i32,
    /// # C++ Class Fields Info
    /// -   name:`"indexBase"`
    /// -   type: `void*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub index_base: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"vertexStriding"`
    /// -   type: `hkUint16`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub vertex_striding: u16,
    /// # C++ Class Fields Info
    /// -   name:`"triangleOffset"`
    /// -   type: `hkInt32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub triangle_offset: i32,
    /// # C++ Class Fields Info
    /// -   name:`"indexStriding"`
    /// -   type: `hkUint16`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub index_striding: u16,
    /// # C++ Class Fields Info
    /// -   name:`"stridingType"`
    /// -   type: `enum IndexStridingType`
    /// - offset: 46
    /// -  flags: `FLAGS_NONE`
    pub striding_type: IndexStridingType,
    /// # C++ Class Fields Info
    /// -   name:`"flipAlternateTriangles"`
    /// -   type: `hkInt8`
    /// - offset: 47
    /// -  flags: `FLAGS_NONE`
    pub flip_alternate_triangles: i8,
    /// # C++ Class Fields Info
    /// -   name:`"extrusion"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub extrusion: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"transform"`
    /// -   type: `hkQsTransform`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub transform: QsTransform<f32>,
}

impl Serialize for HkpExtendedMeshShapeTrianglesSubpart<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpExtendedMeshShapeTrianglesSubpartVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpExtendedMeshShapeTrianglesSubpart<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpExtendedMeshShapeTrianglesSubpartVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpExtendedMeshShapeTrianglesSubpartVisitor<'a>>> for HkpExtendedMeshShapeTrianglesSubpart<'a> {
    fn from(_values: Vec<HkpExtendedMeshShapeTrianglesSubpartVisitor<'a>>) -> Self {
            let mut _type = None;
            let mut material_index_striding_type = None;
            let mut material_striding = None;
            let mut material_index_base = None;
            let mut material_index_striding = None;
            let mut num_materials = None;
            let mut material_base = None;
            let mut user_data = None;
            let mut num_triangle_shapes = None;
            let mut vertex_base = None;
            let mut num_vertices = None;
            let mut index_base = None;
            let mut vertex_striding = None;
            let mut triangle_offset = None;
            let mut index_striding = None;
            let mut striding_type = None;
            let mut flip_alternate_triangles = None;
            let mut extrusion = None;
            let mut transform = None;


        for _value in _values {
            match _value {
                HkpExtendedMeshShapeTrianglesSubpartVisitor::Type(m) => _type = Some(m),
                HkpExtendedMeshShapeTrianglesSubpartVisitor::MaterialIndexStridingType(m) => material_index_striding_type = Some(m),
                HkpExtendedMeshShapeTrianglesSubpartVisitor::MaterialStriding(m) => material_striding = Some(m),
                HkpExtendedMeshShapeTrianglesSubpartVisitor::MaterialIndexBase(m) => material_index_base = Some(m),
                HkpExtendedMeshShapeTrianglesSubpartVisitor::MaterialIndexStriding(m) => material_index_striding = Some(m),
                HkpExtendedMeshShapeTrianglesSubpartVisitor::NumMaterials(m) => num_materials = Some(m),
                HkpExtendedMeshShapeTrianglesSubpartVisitor::MaterialBase(m) => material_base = Some(m),
                HkpExtendedMeshShapeTrianglesSubpartVisitor::UserData(m) => user_data = Some(m),
                HkpExtendedMeshShapeTrianglesSubpartVisitor::NumTriangleShapes(m) => num_triangle_shapes = Some(m),
                HkpExtendedMeshShapeTrianglesSubpartVisitor::VertexBase(m) => vertex_base = Some(m),
                HkpExtendedMeshShapeTrianglesSubpartVisitor::NumVertices(m) => num_vertices = Some(m),
                HkpExtendedMeshShapeTrianglesSubpartVisitor::IndexBase(m) => index_base = Some(m),
                HkpExtendedMeshShapeTrianglesSubpartVisitor::VertexStriding(m) => vertex_striding = Some(m),
                HkpExtendedMeshShapeTrianglesSubpartVisitor::TriangleOffset(m) => triangle_offset = Some(m),
                HkpExtendedMeshShapeTrianglesSubpartVisitor::IndexStriding(m) => index_striding = Some(m),
                HkpExtendedMeshShapeTrianglesSubpartVisitor::StridingType(m) => striding_type = Some(m),
                HkpExtendedMeshShapeTrianglesSubpartVisitor::FlipAlternateTriangles(m) => flip_alternate_triangles = Some(m),
                HkpExtendedMeshShapeTrianglesSubpartVisitor::Extrusion(m) => extrusion = Some(m),
                HkpExtendedMeshShapeTrianglesSubpartVisitor::Transform(m) => transform = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            _type: _type.unwrap_or_default().into_inner(),
            material_index_striding_type: material_index_striding_type.unwrap_or_default().into_inner(),
            material_striding: material_striding.unwrap_or_default().into_inner(),
            material_index_base: material_index_base.unwrap_or_default().into_inner(),
            material_index_striding: material_index_striding.unwrap_or_default().into_inner(),
            num_materials: num_materials.unwrap_or_default().into_inner(),
            material_base: material_base.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            num_triangle_shapes: num_triangle_shapes.unwrap_or_default().into_inner(),
            vertex_base: vertex_base.unwrap_or_default().into_inner(),
            num_vertices: num_vertices.unwrap_or_default().into_inner(),
            index_base: index_base.unwrap_or_default().into_inner(),
            vertex_striding: vertex_striding.unwrap_or_default().into_inner(),
            triangle_offset: triangle_offset.unwrap_or_default().into_inner(),
            index_striding: index_striding.unwrap_or_default().into_inner(),
            striding_type: striding_type.unwrap_or_default().into_inner(),
            flip_alternate_triangles: flip_alternate_triangles.unwrap_or_default().into_inner(),
            extrusion: extrusion.unwrap_or_default().into_inner(),
            transform: transform.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpExtendedMeshShapeTrianglesSubpart<'a>> for Vec<HkpExtendedMeshShapeTrianglesSubpartVisitor<'a>> {
    fn from(data: &HkpExtendedMeshShapeTrianglesSubpart<'a>) -> Self {
        vec![
            HkpExtendedMeshShapeTrianglesSubpartVisitor::Type(data._type.clone().into()),
            HkpExtendedMeshShapeTrianglesSubpartVisitor::MaterialIndexStridingType(data.material_index_striding_type.clone().into()),
            HkpExtendedMeshShapeTrianglesSubpartVisitor::MaterialStriding(data.material_striding.into()),
            HkpExtendedMeshShapeTrianglesSubpartVisitor::MaterialIndexBase(data.material_index_base.clone().into()),
            HkpExtendedMeshShapeTrianglesSubpartVisitor::MaterialIndexStriding(data.material_index_striding.into()),
            HkpExtendedMeshShapeTrianglesSubpartVisitor::NumMaterials(data.num_materials.into()),
            HkpExtendedMeshShapeTrianglesSubpartVisitor::MaterialBase(data.material_base.clone().into()),
            HkpExtendedMeshShapeTrianglesSubpartVisitor::UserData(data.user_data.into()),
            HkpExtendedMeshShapeTrianglesSubpartVisitor::NumTriangleShapes(data.num_triangle_shapes.into()),
            HkpExtendedMeshShapeTrianglesSubpartVisitor::VertexBase(data.vertex_base.clone().into()),
            HkpExtendedMeshShapeTrianglesSubpartVisitor::NumVertices(data.num_vertices.into()),
            HkpExtendedMeshShapeTrianglesSubpartVisitor::IndexBase(data.index_base.clone().into()),
            HkpExtendedMeshShapeTrianglesSubpartVisitor::VertexStriding(data.vertex_striding.into()),
            HkpExtendedMeshShapeTrianglesSubpartVisitor::TriangleOffset(data.triangle_offset.into()),
            HkpExtendedMeshShapeTrianglesSubpartVisitor::IndexStriding(data.index_striding.into()),
            HkpExtendedMeshShapeTrianglesSubpartVisitor::StridingType(data.striding_type.clone().into()),
            HkpExtendedMeshShapeTrianglesSubpartVisitor::FlipAlternateTriangles(data.flip_alternate_triangles.into()),
            HkpExtendedMeshShapeTrianglesSubpartVisitor::Extrusion(data.extrusion.into()),
            HkpExtendedMeshShapeTrianglesSubpartVisitor::Transform(data.transform.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpExtendedMeshShapeTrianglesSubpart<'de> {
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
enum HkpExtendedMeshShapeTrianglesSubpartVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "type")]
    Type(Primitive<SubpartType>),
    /// Visitor fields
    #[serde(rename = "materialIndexStridingType")]
    MaterialIndexStridingType(Primitive<MaterialIndexStridingType>),
    /// Visitor fields
    #[serde(rename = "materialStriding", skip_serializing)]
    MaterialStriding(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "materialIndexBase", skip_serializing)]
    MaterialIndexBase(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "materialIndexStriding")]
    MaterialIndexStriding(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "numMaterials")]
    NumMaterials(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "materialBase", skip_serializing)]
    MaterialBase(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "userData")]
    UserData(Primitive<usize>),

    /// Visitor fields
    #[serde(rename = "numTriangleShapes")]
    NumTriangleShapes(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "vertexBase", skip_serializing)]
    VertexBase(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "numVertices")]
    NumVertices(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "indexBase", skip_serializing)]
    IndexBase(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "vertexStriding")]
    VertexStriding(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "triangleOffset")]
    TriangleOffset(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "indexStriding")]
    IndexStriding(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "stridingType")]
    StridingType(Primitive<IndexStridingType>),
    /// Visitor fields
    #[serde(rename = "flipAlternateTriangles")]
    FlipAlternateTriangles(Primitive<i8>),
    /// Visitor fields
    #[serde(rename = "extrusion")]
    Extrusion(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "transform")]
    Transform(Primitive<QsTransform<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpExtendedMeshShapeTrianglesSubpartVisitor<'de>, "@name",
    ("type" => Type(Primitive<SubpartType>)),
    ("materialIndexStridingType" => MaterialIndexStridingType(Primitive<MaterialIndexStridingType>)),
    ("materialStriding" => MaterialStriding(Primitive<i16>)),
    ("materialIndexBase" => MaterialIndexBase(Primitive<Cow<'de, str>>)),
    ("materialIndexStriding" => MaterialIndexStriding(Primitive<u16>)),
    ("numMaterials" => NumMaterials(Primitive<u16>)),
    ("materialBase" => MaterialBase(Primitive<Cow<'de, str>>)),
    ("userData" => UserData(Primitive<usize>)),
    ("numTriangleShapes" => NumTriangleShapes(Primitive<i32>)),
    ("vertexBase" => VertexBase(Primitive<Cow<'de, str>>)),
    ("numVertices" => NumVertices(Primitive<i32>)),
    ("indexBase" => IndexBase(Primitive<Cow<'de, str>>)),
    ("vertexStriding" => VertexStriding(Primitive<u16>)),
    ("triangleOffset" => TriangleOffset(Primitive<i32>)),
    ("indexStriding" => IndexStriding(Primitive<u16>)),
    ("stridingType" => StridingType(Primitive<IndexStridingType>)),
    ("flipAlternateTriangles" => FlipAlternateTriangles(Primitive<i8>)),
    ("extrusion" => Extrusion(Primitive<Vector4<f32>>)),
    ("transform" => Transform(Primitive<QsTransform<f32>>)),
}
