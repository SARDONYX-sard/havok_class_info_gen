//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCompressedMeshShape`
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

/// `hkpCompressedMeshShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 224
/// -    vtable: true
/// -    parent: `hkpShapeCollection`/`0xe8c3991d`
/// - signature: `0xa62d5e6e`
/// -   version: 9
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpCompressedMeshShape<'a> {
    /// # C++ Parent class(`hkpShapeCollection` => parent: `hkpShape`) field Info
    /// -   name:`"disableWelding"`
    /// -   type: `hkBool`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub disable_welding: bool,
    /// # C++ Parent class(`hkpShapeCollection` => parent: `hkpShape`) field Info
    /// -   name:`"collectionType"`
    /// -   type: `enum CollectionType`
    /// - offset: 21
    /// -  flags: `FLAGS_NONE`
    pub collection_type: CollectionType,

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
    /// -   name:`"bitsPerIndex"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub bits_per_index: i32,
    /// # C++ Class Fields Info
    /// -   name:`"bitsPerWIndex"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub bits_per_w_index: i32,
    /// # C++ Class Fields Info
    /// -   name:`"wIndexMask"`
    /// -   type: `hkInt32`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub w_index_mask: i32,
    /// # C++ Class Fields Info
    /// -   name:`"indexMask"`
    /// -   type: `hkInt32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub index_mask: i32,
    /// # C++ Class Fields Info
    /// -   name:`"radius"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub radius: f32,
    /// # C++ Class Fields Info
    /// -   name:`"weldingType"`
    /// -   type: `enum WeldingType`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub welding_type: WeldingType,
    /// # C++ Class Fields Info
    /// -   name:`"materialType"`
    /// -   type: `enum MaterialType`
    /// - offset: 45
    /// -  flags: `FLAGS_NONE`
    pub material_type: MaterialType,
    /// # C++ Class Fields Info
    /// -   name:`"materials"`
    /// -   type: `hkArray<hkUint32>`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub materials: HkArrayNum<u32>,
    /// # C++ Class Fields Info
    /// -   name:`"materials16"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub materials_16: HkArrayNum<u16>,
    /// # C++ Class Fields Info
    /// -   name:`"materials8"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    pub materials_8: HkArrayNum<u8>,
    /// # C++ Class Fields Info
    /// -   name:`"transforms"`
    /// -   type: `hkArray<hkQsTransform>`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    pub transforms: HkArrayMatrix3<QsTransform<f32>>,
    /// # C++ Class Fields Info
    /// -   name:`"bigVertices"`
    /// -   type: `hkArray<hkVector4>`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub big_vertices: HkArrayVector<Vector4<f32>>,
    /// # C++ Class Fields Info
    /// -   name:`"bigTriangles"`
    /// -   type: `hkArray<struct hkpCompressedMeshShapeBigTriangle>`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    pub big_triangles: HkArrayClass<HkpCompressedMeshShapeBigTriangle>,
    /// # C++ Class Fields Info
    /// -   name:`"chunks"`
    /// -   type: `hkArray<struct hkpCompressedMeshShapeChunk>`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    pub chunks: HkArrayClass<HkpCompressedMeshShapeChunk>,
    /// # C++ Class Fields Info
    /// -   name:`"convexPieces"`
    /// -   type: `hkArray<struct hkpCompressedMeshShapeConvexPiece>`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE`
    pub convex_pieces: HkArrayClass<HkpCompressedMeshShapeConvexPiece>,
    /// # C++ Class Fields Info
    /// -   name:`"error"`
    /// -   type: `hkReal`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    pub error: f32,
    /// # C++ Class Fields Info
    /// -   name:`"bounds"`
    /// -   type: `struct hkAabb`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    pub bounds: SingleClass<HkAabb>,
    /// # C++ Class Fields Info
    /// -   name:`"defaultCollisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    pub default_collision_filter_info: u32,
    /// # C++ Class Fields Info
    /// -   name:`"meshMaterials"`
    /// -   type: `void*`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub mesh_materials: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"materialStriding"`
    /// -   type: `hkUint16`
    /// - offset: 200
    /// -  flags: `FLAGS_NONE`
    pub material_striding: u16,
    /// # C++ Class Fields Info
    /// -   name:`"numMaterials"`
    /// -   type: `hkUint16`
    /// - offset: 202
    /// -  flags: `FLAGS_NONE`
    pub num_materials: u16,
    /// # C++ Class Fields Info
    /// -   name:`"namedMaterials"`
    /// -   type: `hkArray<struct hkpNamedMeshMaterial>`
    /// - offset: 204
    /// -  flags: `FLAGS_NONE`
    pub named_materials: HkArrayClass<HkpNamedMeshMaterial<'a>>,
}

impl Serialize for HkpCompressedMeshShape<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpCompressedMeshShapeVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpCompressedMeshShape<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpCompressedMeshShapeVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpCompressedMeshShapeVisitor<'a>>> for HkpCompressedMeshShape<'a> {
    fn from(_values: Vec<HkpCompressedMeshShapeVisitor<'a>>) -> Self {
            let mut disable_welding = None;
            let mut collection_type = None;
            let mut user_data = None;
            let mut _type = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut bits_per_index = None;
            let mut bits_per_w_index = None;
            let mut w_index_mask = None;
            let mut index_mask = None;
            let mut radius = None;
            let mut welding_type = None;
            let mut material_type = None;
            let mut materials = None;
            let mut materials_16 = None;
            let mut materials_8 = None;
            let mut transforms = None;
            let mut big_vertices = None;
            let mut big_triangles = None;
            let mut chunks = None;
            let mut convex_pieces = None;
            let mut error = None;
            let mut bounds = None;
            let mut default_collision_filter_info = None;
            let mut mesh_materials = None;
            let mut material_striding = None;
            let mut num_materials = None;
            let mut named_materials = None;


        for _value in _values {
            match _value {
                HkpCompressedMeshShapeVisitor::DisableWelding(m) => disable_welding = Some(m),
                HkpCompressedMeshShapeVisitor::CollectionType(m) => collection_type = Some(m),
                HkpCompressedMeshShapeVisitor::UserData(m) => user_data = Some(m),
                HkpCompressedMeshShapeVisitor::Type(m) => _type = Some(m),
                HkpCompressedMeshShapeVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpCompressedMeshShapeVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpCompressedMeshShapeVisitor::BitsPerIndex(m) => bits_per_index = Some(m),
                HkpCompressedMeshShapeVisitor::BitsPerWIndex(m) => bits_per_w_index = Some(m),
                HkpCompressedMeshShapeVisitor::WIndexMask(m) => w_index_mask = Some(m),
                HkpCompressedMeshShapeVisitor::IndexMask(m) => index_mask = Some(m),
                HkpCompressedMeshShapeVisitor::Radius(m) => radius = Some(m),
                HkpCompressedMeshShapeVisitor::WeldingType(m) => welding_type = Some(m),
                HkpCompressedMeshShapeVisitor::MaterialType(m) => material_type = Some(m),
                HkpCompressedMeshShapeVisitor::Materials(m) => materials = Some(m),
                HkpCompressedMeshShapeVisitor::Materials16(m) => materials_16 = Some(m),
                HkpCompressedMeshShapeVisitor::Materials8(m) => materials_8 = Some(m),
                HkpCompressedMeshShapeVisitor::Transforms(m) => transforms = Some(m),
                HkpCompressedMeshShapeVisitor::BigVertices(m) => big_vertices = Some(m),
                HkpCompressedMeshShapeVisitor::BigTriangles(m) => big_triangles = Some(m),
                HkpCompressedMeshShapeVisitor::Chunks(m) => chunks = Some(m),
                HkpCompressedMeshShapeVisitor::ConvexPieces(m) => convex_pieces = Some(m),
                HkpCompressedMeshShapeVisitor::Error(m) => error = Some(m),
                HkpCompressedMeshShapeVisitor::Bounds(m) => bounds = Some(m),
                HkpCompressedMeshShapeVisitor::DefaultCollisionFilterInfo(m) => default_collision_filter_info = Some(m),
                HkpCompressedMeshShapeVisitor::MeshMaterials(m) => mesh_materials = Some(m),
                HkpCompressedMeshShapeVisitor::MaterialStriding(m) => material_striding = Some(m),
                HkpCompressedMeshShapeVisitor::NumMaterials(m) => num_materials = Some(m),
                HkpCompressedMeshShapeVisitor::NamedMaterials(m) => named_materials = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            disable_welding: disable_welding.unwrap_or_default().into_inner(),
            collection_type: collection_type.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            bits_per_index: bits_per_index.unwrap_or_default().into_inner(),
            bits_per_w_index: bits_per_w_index.unwrap_or_default().into_inner(),
            w_index_mask: w_index_mask.unwrap_or_default().into_inner(),
            index_mask: index_mask.unwrap_or_default().into_inner(),
            radius: radius.unwrap_or_default().into_inner(),
            welding_type: welding_type.unwrap_or_default().into_inner(),
            material_type: material_type.unwrap_or_default().into_inner(),
            materials: materials.unwrap_or_default(),
            materials_16: materials_16.unwrap_or_default(),
            materials_8: materials_8.unwrap_or_default(),
            transforms: transforms.unwrap_or_default(),
            big_vertices: big_vertices.unwrap_or_default(),
            big_triangles: big_triangles.unwrap_or_default(),
            chunks: chunks.unwrap_or_default(),
            convex_pieces: convex_pieces.unwrap_or_default(),
            error: error.unwrap_or_default().into_inner(),
            bounds: bounds.unwrap_or_default(),
            default_collision_filter_info: default_collision_filter_info.unwrap_or_default().into_inner(),
            mesh_materials: mesh_materials.unwrap_or_default().into_inner(),
            material_striding: material_striding.unwrap_or_default().into_inner(),
            num_materials: num_materials.unwrap_or_default().into_inner(),
            named_materials: named_materials.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpCompressedMeshShape<'a>> for Vec<HkpCompressedMeshShapeVisitor<'a>> {
    fn from(data: &HkpCompressedMeshShape<'a>) -> Self {
        vec![
            HkpCompressedMeshShapeVisitor::DisableWelding(data.disable_welding.into()),
            HkpCompressedMeshShapeVisitor::CollectionType(data.collection_type.clone().into()),
            HkpCompressedMeshShapeVisitor::UserData(data.user_data.into()),
            HkpCompressedMeshShapeVisitor::Type(data._type.into()),
            HkpCompressedMeshShapeVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpCompressedMeshShapeVisitor::ReferenceCount(data.reference_count.into()),
            HkpCompressedMeshShapeVisitor::BitsPerIndex(data.bits_per_index.into()),
            HkpCompressedMeshShapeVisitor::BitsPerWIndex(data.bits_per_w_index.into()),
            HkpCompressedMeshShapeVisitor::WIndexMask(data.w_index_mask.into()),
            HkpCompressedMeshShapeVisitor::IndexMask(data.index_mask.into()),
            HkpCompressedMeshShapeVisitor::Radius(data.radius.into()),
            HkpCompressedMeshShapeVisitor::WeldingType(data.welding_type.clone().into()),
            HkpCompressedMeshShapeVisitor::MaterialType(data.material_type.clone().into()),
            HkpCompressedMeshShapeVisitor::Materials(data.materials.clone()),
            HkpCompressedMeshShapeVisitor::Materials16(data.materials_16.clone()),
            HkpCompressedMeshShapeVisitor::Materials8(data.materials_8.clone()),
            HkpCompressedMeshShapeVisitor::Transforms(data.transforms.clone()),
            HkpCompressedMeshShapeVisitor::BigVertices(data.big_vertices.clone()),
            HkpCompressedMeshShapeVisitor::BigTriangles(data.big_triangles.clone()),
            HkpCompressedMeshShapeVisitor::Chunks(data.chunks.clone()),
            HkpCompressedMeshShapeVisitor::ConvexPieces(data.convex_pieces.clone()),
            HkpCompressedMeshShapeVisitor::Error(data.error.into()),
            HkpCompressedMeshShapeVisitor::Bounds(data.bounds.clone()),
            HkpCompressedMeshShapeVisitor::DefaultCollisionFilterInfo(data.default_collision_filter_info.into()),
            HkpCompressedMeshShapeVisitor::MeshMaterials(data.mesh_materials.clone().into()),
            HkpCompressedMeshShapeVisitor::MaterialStriding(data.material_striding.into()),
            HkpCompressedMeshShapeVisitor::NumMaterials(data.num_materials.into()),
            HkpCompressedMeshShapeVisitor::NamedMaterials(data.named_materials.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpCompressedMeshShape<'de> {
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
enum HkpCompressedMeshShapeVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "disableWelding")]
    DisableWelding(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "collectionType")]
    CollectionType(Primitive<CollectionType>),

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
    #[serde(rename = "bitsPerIndex")]
    BitsPerIndex(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "bitsPerWIndex")]
    BitsPerWIndex(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "wIndexMask")]
    WIndexMask(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "indexMask")]
    IndexMask(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "radius")]
    Radius(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "weldingType")]
    WeldingType(Primitive<WeldingType>),
    /// Visitor fields
    #[serde(rename = "materialType")]
    MaterialType(Primitive<MaterialType>),
    /// Visitor fields
    #[serde(rename = "materials")]
    Materials(HkArrayNum<u32>),
    /// Visitor fields
    #[serde(rename = "materials16")]
    Materials16(HkArrayNum<u16>),
    /// Visitor fields
    #[serde(rename = "materials8")]
    Materials8(HkArrayNum<u8>),
    /// Visitor fields
    #[serde(rename = "transforms")]
    Transforms(HkArrayMatrix3<QsTransform<f32>>),
    /// Visitor fields
    #[serde(rename = "bigVertices")]
    BigVertices(HkArrayVector<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "bigTriangles")]
    BigTriangles(HkArrayClass<HkpCompressedMeshShapeBigTriangle>),
    /// Visitor fields
    #[serde(rename = "chunks")]
    Chunks(HkArrayClass<HkpCompressedMeshShapeChunk>),
    /// Visitor fields
    #[serde(rename = "convexPieces")]
    ConvexPieces(HkArrayClass<HkpCompressedMeshShapeConvexPiece>),
    /// Visitor fields
    #[serde(rename = "error")]
    Error(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "bounds")]
    Bounds(SingleClass<HkAabb>),
    /// Visitor fields
    #[serde(rename = "defaultCollisionFilterInfo")]
    DefaultCollisionFilterInfo(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "meshMaterials", skip_serializing)]
    MeshMaterials(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "materialStriding")]
    MaterialStriding(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "numMaterials")]
    NumMaterials(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "namedMaterials")]
    NamedMaterials(HkArrayClass<HkpNamedMeshMaterial<'a>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCompressedMeshShapeVisitor<'de>, "@name",
    ("disableWelding" => DisableWelding(Primitive<bool>)),
    ("collectionType" => CollectionType(Primitive<CollectionType>)),
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("bitsPerIndex" => BitsPerIndex(Primitive<i32>)),
    ("bitsPerWIndex" => BitsPerWIndex(Primitive<i32>)),
    ("wIndexMask" => WIndexMask(Primitive<i32>)),
    ("indexMask" => IndexMask(Primitive<i32>)),
    ("radius" => Radius(Primitive<f32>)),
    ("weldingType" => WeldingType(Primitive<WeldingType>)),
    ("materialType" => MaterialType(Primitive<MaterialType>)),
    ("materials" => Materials(HkArrayNum<u32>)),
    ("materials16" => Materials16(HkArrayNum<u16>)),
    ("materials8" => Materials8(HkArrayNum<u8>)),
    ("transforms" => Transforms(HkArrayMatrix3<QsTransform<f32>>)),
    ("bigVertices" => BigVertices(HkArrayVector<Vector4<f32>>)),
    ("bigTriangles" => BigTriangles(HkArrayClass<HkpCompressedMeshShapeBigTriangle>)),
    ("chunks" => Chunks(HkArrayClass<HkpCompressedMeshShapeChunk>)),
    ("convexPieces" => ConvexPieces(HkArrayClass<HkpCompressedMeshShapeConvexPiece>)),
    ("error" => Error(Primitive<f32>)),
    ("bounds" => Bounds(SingleClass<HkAabb>)),
    ("defaultCollisionFilterInfo" => DefaultCollisionFilterInfo(Primitive<u32>)),
    ("meshMaterials" => MeshMaterials(Primitive<Cow<'de, str>>)),
    ("materialStriding" => MaterialStriding(Primitive<u16>)),
    ("numMaterials" => NumMaterials(Primitive<u16>)),
    ("namedMaterials" => NamedMaterials(HkArrayClass<HkpNamedMeshMaterial<'de>>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum MaterialType {
    #[serde(rename = "MATERIAL_NONE")]
    #[default]
    MaterialNone = 0,
    #[serde(rename = "MATERIAL_SINGLE_VALUE_PER_CHUNK")]
    MaterialSingleValuePerChunk = 1,
    #[serde(rename = "MATERIAL_ONE_BYTE_PER_TRIANGLE")]
    MaterialOneBytePerTriangle = 2,
    #[serde(rename = "MATERIAL_TWO_BYTES_PER_TRIANGLE")]
    MaterialTwoBytesPerTriangle = 3,
    #[serde(rename = "MATERIAL_FOUR_BYTES_PER_TRIANGLE")]
    MaterialFourBytesPerTriangle = 4,
}
