//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpMeshShapeSubpart`
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

/// `hkpMeshShapeSubpart`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 56
/// -    vtable: false
/// - signature: `0x27336e5d`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpMeshShapeSubpart<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"vertexBase"`
    /// -   type: `void*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub vertex_base: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"vertexStriding"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub vertex_striding: i32,
    /// # C++ Class Fields Info
    /// -   name:`"numVertices"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub num_vertices: i32,
    /// # C++ Class Fields Info
    /// -   name:`"indexBase"`
    /// -   type: `void*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub index_base: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"stridingType"`
    /// -   type: `enum MeshShapeIndexStridingType`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub striding_type: MeshShapeIndexStridingType,
    /// # C++ Class Fields Info
    /// -   name:`"materialIndexStridingType"`
    /// -   type: `enum MeshShapeMaterialIndexStridingType`
    /// - offset: 17
    /// -  flags: `FLAGS_NONE`
    pub material_index_striding_type: MeshShapeMaterialIndexStridingType,
    /// # C++ Class Fields Info
    /// -   name:`"indexStriding"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub index_striding: i32,
    /// # C++ Class Fields Info
    /// -   name:`"flipAlternateTriangles"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub flip_alternate_triangles: i32,
    /// # C++ Class Fields Info
    /// -   name:`"numTriangles"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub num_triangles: i32,
    /// # C++ Class Fields Info
    /// -   name:`"materialIndexBase"`
    /// -   type: `void*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub material_index_base: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"materialIndexStriding"`
    /// -   type: `hkInt32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub material_index_striding: i32,
    /// # C++ Class Fields Info
    /// -   name:`"materialBase"`
    /// -   type: `void*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub material_base: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"materialStriding"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub material_striding: i32,
    /// # C++ Class Fields Info
    /// -   name:`"numMaterials"`
    /// -   type: `hkInt32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub num_materials: i32,
    /// # C++ Class Fields Info
    /// -   name:`"triangleOffset"`
    /// -   type: `hkInt32`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub triangle_offset: i32,
}

impl Serialize for HkpMeshShapeSubpart<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpMeshShapeSubpartVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpMeshShapeSubpart<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpMeshShapeSubpartVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpMeshShapeSubpartVisitor<'a>>> for HkpMeshShapeSubpart<'a> {
    fn from(_values: Vec<HkpMeshShapeSubpartVisitor<'a>>) -> Self {
            let mut vertex_base = None;
            let mut vertex_striding = None;
            let mut num_vertices = None;
            let mut index_base = None;
            let mut striding_type = None;
            let mut material_index_striding_type = None;
            let mut index_striding = None;
            let mut flip_alternate_triangles = None;
            let mut num_triangles = None;
            let mut material_index_base = None;
            let mut material_index_striding = None;
            let mut material_base = None;
            let mut material_striding = None;
            let mut num_materials = None;
            let mut triangle_offset = None;


        for _value in _values {
            match _value {
                HkpMeshShapeSubpartVisitor::VertexBase(m) => vertex_base = Some(m),
                HkpMeshShapeSubpartVisitor::VertexStriding(m) => vertex_striding = Some(m),
                HkpMeshShapeSubpartVisitor::NumVertices(m) => num_vertices = Some(m),
                HkpMeshShapeSubpartVisitor::IndexBase(m) => index_base = Some(m),
                HkpMeshShapeSubpartVisitor::StridingType(m) => striding_type = Some(m),
                HkpMeshShapeSubpartVisitor::MaterialIndexStridingType(m) => material_index_striding_type = Some(m),
                HkpMeshShapeSubpartVisitor::IndexStriding(m) => index_striding = Some(m),
                HkpMeshShapeSubpartVisitor::FlipAlternateTriangles(m) => flip_alternate_triangles = Some(m),
                HkpMeshShapeSubpartVisitor::NumTriangles(m) => num_triangles = Some(m),
                HkpMeshShapeSubpartVisitor::MaterialIndexBase(m) => material_index_base = Some(m),
                HkpMeshShapeSubpartVisitor::MaterialIndexStriding(m) => material_index_striding = Some(m),
                HkpMeshShapeSubpartVisitor::MaterialBase(m) => material_base = Some(m),
                HkpMeshShapeSubpartVisitor::MaterialStriding(m) => material_striding = Some(m),
                HkpMeshShapeSubpartVisitor::NumMaterials(m) => num_materials = Some(m),
                HkpMeshShapeSubpartVisitor::TriangleOffset(m) => triangle_offset = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            vertex_base: vertex_base.unwrap_or_default().into_inner(),
            vertex_striding: vertex_striding.unwrap_or_default().into_inner(),
            num_vertices: num_vertices.unwrap_or_default().into_inner(),
            index_base: index_base.unwrap_or_default().into_inner(),
            striding_type: striding_type.unwrap_or_default().into_inner(),
            material_index_striding_type: material_index_striding_type.unwrap_or_default().into_inner(),
            index_striding: index_striding.unwrap_or_default().into_inner(),
            flip_alternate_triangles: flip_alternate_triangles.unwrap_or_default().into_inner(),
            num_triangles: num_triangles.unwrap_or_default().into_inner(),
            material_index_base: material_index_base.unwrap_or_default().into_inner(),
            material_index_striding: material_index_striding.unwrap_or_default().into_inner(),
            material_base: material_base.unwrap_or_default().into_inner(),
            material_striding: material_striding.unwrap_or_default().into_inner(),
            num_materials: num_materials.unwrap_or_default().into_inner(),
            triangle_offset: triangle_offset.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpMeshShapeSubpart<'a>> for Vec<HkpMeshShapeSubpartVisitor<'a>> {
    fn from(data: &HkpMeshShapeSubpart<'a>) -> Self {
        vec![
            HkpMeshShapeSubpartVisitor::VertexBase(data.vertex_base.clone().into()),
            HkpMeshShapeSubpartVisitor::VertexStriding(data.vertex_striding.into()),
            HkpMeshShapeSubpartVisitor::NumVertices(data.num_vertices.into()),
            HkpMeshShapeSubpartVisitor::IndexBase(data.index_base.clone().into()),
            HkpMeshShapeSubpartVisitor::StridingType(data.striding_type.clone().into()),
            HkpMeshShapeSubpartVisitor::MaterialIndexStridingType(data.material_index_striding_type.clone().into()),
            HkpMeshShapeSubpartVisitor::IndexStriding(data.index_striding.into()),
            HkpMeshShapeSubpartVisitor::FlipAlternateTriangles(data.flip_alternate_triangles.into()),
            HkpMeshShapeSubpartVisitor::NumTriangles(data.num_triangles.into()),
            HkpMeshShapeSubpartVisitor::MaterialIndexBase(data.material_index_base.clone().into()),
            HkpMeshShapeSubpartVisitor::MaterialIndexStriding(data.material_index_striding.into()),
            HkpMeshShapeSubpartVisitor::MaterialBase(data.material_base.clone().into()),
            HkpMeshShapeSubpartVisitor::MaterialStriding(data.material_striding.into()),
            HkpMeshShapeSubpartVisitor::NumMaterials(data.num_materials.into()),
            HkpMeshShapeSubpartVisitor::TriangleOffset(data.triangle_offset.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpMeshShapeSubpart<'de> {
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
enum HkpMeshShapeSubpartVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "vertexBase", skip_serializing)]
    VertexBase(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "vertexStriding")]
    VertexStriding(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "numVertices")]
    NumVertices(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "indexBase", skip_serializing)]
    IndexBase(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "stridingType")]
    StridingType(Primitive<MeshShapeIndexStridingType>),
    /// Visitor fields
    #[serde(rename = "materialIndexStridingType")]
    MaterialIndexStridingType(Primitive<MeshShapeMaterialIndexStridingType>),
    /// Visitor fields
    #[serde(rename = "indexStriding")]
    IndexStriding(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "flipAlternateTriangles")]
    FlipAlternateTriangles(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "numTriangles")]
    NumTriangles(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "materialIndexBase", skip_serializing)]
    MaterialIndexBase(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "materialIndexStriding")]
    MaterialIndexStriding(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "materialBase", skip_serializing)]
    MaterialBase(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "materialStriding")]
    MaterialStriding(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "numMaterials")]
    NumMaterials(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "triangleOffset")]
    TriangleOffset(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpMeshShapeSubpartVisitor<'de>, "@name",
    ("vertexBase" => VertexBase(Primitive<Cow<'de, str>>)),
    ("vertexStriding" => VertexStriding(Primitive<i32>)),
    ("numVertices" => NumVertices(Primitive<i32>)),
    ("indexBase" => IndexBase(Primitive<Cow<'de, str>>)),
    ("stridingType" => StridingType(Primitive<MeshShapeIndexStridingType>)),
    ("materialIndexStridingType" => MaterialIndexStridingType(Primitive<MeshShapeMaterialIndexStridingType>)),
    ("indexStriding" => IndexStriding(Primitive<i32>)),
    ("flipAlternateTriangles" => FlipAlternateTriangles(Primitive<i32>)),
    ("numTriangles" => NumTriangles(Primitive<i32>)),
    ("materialIndexBase" => MaterialIndexBase(Primitive<Cow<'de, str>>)),
    ("materialIndexStriding" => MaterialIndexStriding(Primitive<i32>)),
    ("materialBase" => MaterialBase(Primitive<Cow<'de, str>>)),
    ("materialStriding" => MaterialStriding(Primitive<i32>)),
    ("numMaterials" => NumMaterials(Primitive<i32>)),
    ("triangleOffset" => TriangleOffset(Primitive<i32>)),
}
