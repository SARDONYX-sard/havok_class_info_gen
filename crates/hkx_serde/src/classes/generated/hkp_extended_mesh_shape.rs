//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpExtendedMeshShape`
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

/// `hkpExtendedMeshShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 240
/// -    vtable: true
/// -    parent: `hkpShapeCollection`/`0xe8c3991d`
/// - signature: `0x177114a2`
/// -   version: 3
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpExtendedMeshShape<'a> {
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
    /// -   name:`"embeddedTrianglesSubpart"`
    /// -   type: `struct hkpExtendedMeshShapeTrianglesSubpart`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub embedded_triangles_subpart: SingleClass<HkpExtendedMeshShapeTrianglesSubpart<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"aabbHalfExtents"`
    /// -   type: `hkVector4`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    pub aabb_half_extents: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"aabbCenter"`
    /// -   type: `hkVector4`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    pub aabb_center: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"materialClass"`
    /// -   type: `void*`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub material_class: Cow<'a, str>,
    /// # C++ Class Fields Info
    /// -   name:`"numBitsForSubpartIndex"`
    /// -   type: `hkInt32`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    pub num_bits_for_subpart_index: i32,
    /// # C++ Class Fields Info
    /// -   name:`"trianglesSubparts"`
    /// -   type: `hkArray<struct hkpExtendedMeshShapeTrianglesSubpart>`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE`
    pub triangles_subparts: HkArrayClass<HkpExtendedMeshShapeTrianglesSubpart<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"shapesSubparts"`
    /// -   type: `hkArray<struct hkpExtendedMeshShapeShapesSubpart>`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE`
    pub shapes_subparts: HkArrayClass<HkpExtendedMeshShapeShapesSubpart<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"weldingInfo"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    pub welding_info: HkArrayNum<u16>,
    /// # C++ Class Fields Info
    /// -   name:`"weldingType"`
    /// -   type: `enum WeldingType`
    /// - offset: 220
    /// -  flags: `FLAGS_NONE`
    pub welding_type: WeldingType,
    /// # C++ Class Fields Info
    /// -   name:`"defaultCollisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 224
    /// -  flags: `FLAGS_NONE`
    pub default_collision_filter_info: u32,
    /// # C++ Class Fields Info
    /// -   name:`"cachedNumChildShapes"`
    /// -   type: `hkInt32`
    /// - offset: 228
    /// -  flags: `FLAGS_NONE`
    pub cached_num_child_shapes: i32,
    /// # C++ Class Fields Info
    /// -   name:`"triangleRadius"`
    /// -   type: `hkReal`
    /// - offset: 232
    /// -  flags: `FLAGS_NONE`
    pub triangle_radius: f32,
    /// # C++ Class Fields Info
    /// -   name:`"padding"`
    /// -   type: `hkInt32`
    /// - offset: 236
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub padding: i32,
}

impl Serialize for HkpExtendedMeshShape<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpExtendedMeshShapeVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpExtendedMeshShape<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpExtendedMeshShapeVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpExtendedMeshShapeVisitor<'a>>> for HkpExtendedMeshShape<'a> {
    fn from(_values: Vec<HkpExtendedMeshShapeVisitor<'a>>) -> Self {
            let mut disable_welding = None;
            let mut collection_type = None;
            let mut user_data = None;
            let mut _type = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut embedded_triangles_subpart = None;
            let mut aabb_half_extents = None;
            let mut aabb_center = None;
            let mut material_class = None;
            let mut num_bits_for_subpart_index = None;
            let mut triangles_subparts = None;
            let mut shapes_subparts = None;
            let mut welding_info = None;
            let mut welding_type = None;
            let mut default_collision_filter_info = None;
            let mut cached_num_child_shapes = None;
            let mut triangle_radius = None;
            let mut padding = None;


        for _value in _values {
            match _value {
                HkpExtendedMeshShapeVisitor::DisableWelding(m) => disable_welding = Some(m),
                HkpExtendedMeshShapeVisitor::CollectionType(m) => collection_type = Some(m),
                HkpExtendedMeshShapeVisitor::UserData(m) => user_data = Some(m),
                HkpExtendedMeshShapeVisitor::Type(m) => _type = Some(m),
                HkpExtendedMeshShapeVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpExtendedMeshShapeVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpExtendedMeshShapeVisitor::EmbeddedTrianglesSubpart(m) => embedded_triangles_subpart = Some(m),
                HkpExtendedMeshShapeVisitor::AabbHalfExtents(m) => aabb_half_extents = Some(m),
                HkpExtendedMeshShapeVisitor::AabbCenter(m) => aabb_center = Some(m),
                HkpExtendedMeshShapeVisitor::MaterialClass(m) => material_class = Some(m),
                HkpExtendedMeshShapeVisitor::NumBitsForSubpartIndex(m) => num_bits_for_subpart_index = Some(m),
                HkpExtendedMeshShapeVisitor::TrianglesSubparts(m) => triangles_subparts = Some(m),
                HkpExtendedMeshShapeVisitor::ShapesSubparts(m) => shapes_subparts = Some(m),
                HkpExtendedMeshShapeVisitor::WeldingInfo(m) => welding_info = Some(m),
                HkpExtendedMeshShapeVisitor::WeldingType(m) => welding_type = Some(m),
                HkpExtendedMeshShapeVisitor::DefaultCollisionFilterInfo(m) => default_collision_filter_info = Some(m),
                HkpExtendedMeshShapeVisitor::CachedNumChildShapes(m) => cached_num_child_shapes = Some(m),
                HkpExtendedMeshShapeVisitor::TriangleRadius(m) => triangle_radius = Some(m),
                HkpExtendedMeshShapeVisitor::Padding(m) => padding = Some(m),

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
            embedded_triangles_subpart: embedded_triangles_subpart.unwrap_or_default(),
            aabb_half_extents: aabb_half_extents.unwrap_or_default().into_inner(),
            aabb_center: aabb_center.unwrap_or_default().into_inner(),
            material_class: material_class.unwrap_or_default().into_inner(),
            num_bits_for_subpart_index: num_bits_for_subpart_index.unwrap_or_default().into_inner(),
            triangles_subparts: triangles_subparts.unwrap_or_default(),
            shapes_subparts: shapes_subparts.unwrap_or_default(),
            welding_info: welding_info.unwrap_or_default(),
            welding_type: welding_type.unwrap_or_default().into_inner(),
            default_collision_filter_info: default_collision_filter_info.unwrap_or_default().into_inner(),
            cached_num_child_shapes: cached_num_child_shapes.unwrap_or_default().into_inner(),
            triangle_radius: triangle_radius.unwrap_or_default().into_inner(),
            padding: padding.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpExtendedMeshShape<'a>> for Vec<HkpExtendedMeshShapeVisitor<'a>> {
    fn from(data: &HkpExtendedMeshShape<'a>) -> Self {
        vec![
            HkpExtendedMeshShapeVisitor::DisableWelding(data.disable_welding.into()),
            HkpExtendedMeshShapeVisitor::CollectionType(data.collection_type.clone().into()),
            HkpExtendedMeshShapeVisitor::UserData(data.user_data.into()),
            HkpExtendedMeshShapeVisitor::Type(data._type.into()),
            HkpExtendedMeshShapeVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpExtendedMeshShapeVisitor::ReferenceCount(data.reference_count.into()),
            HkpExtendedMeshShapeVisitor::EmbeddedTrianglesSubpart(data.embedded_triangles_subpart.clone()),
            HkpExtendedMeshShapeVisitor::AabbHalfExtents(data.aabb_half_extents.into()),
            HkpExtendedMeshShapeVisitor::AabbCenter(data.aabb_center.into()),
            HkpExtendedMeshShapeVisitor::MaterialClass(data.material_class.clone().into()),
            HkpExtendedMeshShapeVisitor::NumBitsForSubpartIndex(data.num_bits_for_subpart_index.into()),
            HkpExtendedMeshShapeVisitor::TrianglesSubparts(data.triangles_subparts.clone()),
            HkpExtendedMeshShapeVisitor::ShapesSubparts(data.shapes_subparts.clone()),
            HkpExtendedMeshShapeVisitor::WeldingInfo(data.welding_info.clone()),
            HkpExtendedMeshShapeVisitor::WeldingType(data.welding_type.clone().into()),
            HkpExtendedMeshShapeVisitor::DefaultCollisionFilterInfo(data.default_collision_filter_info.into()),
            HkpExtendedMeshShapeVisitor::CachedNumChildShapes(data.cached_num_child_shapes.into()),
            HkpExtendedMeshShapeVisitor::TriangleRadius(data.triangle_radius.into()),
            HkpExtendedMeshShapeVisitor::Padding(data.padding.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpExtendedMeshShape<'de> {
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
enum HkpExtendedMeshShapeVisitor<'a> {
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
    #[serde(rename = "embeddedTrianglesSubpart")]
    EmbeddedTrianglesSubpart(SingleClass<HkpExtendedMeshShapeTrianglesSubpart<'a>>),
    /// Visitor fields
    #[serde(rename = "aabbHalfExtents")]
    AabbHalfExtents(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "aabbCenter")]
    AabbCenter(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "materialClass", skip_serializing)]
    MaterialClass(Primitive<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "numBitsForSubpartIndex")]
    NumBitsForSubpartIndex(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "trianglesSubparts")]
    TrianglesSubparts(HkArrayClass<HkpExtendedMeshShapeTrianglesSubpart<'a>>),
    /// Visitor fields
    #[serde(rename = "shapesSubparts")]
    ShapesSubparts(HkArrayClass<HkpExtendedMeshShapeShapesSubpart<'a>>),
    /// Visitor fields
    #[serde(rename = "weldingInfo")]
    WeldingInfo(HkArrayNum<u16>),
    /// Visitor fields
    #[serde(rename = "weldingType")]
    WeldingType(Primitive<WeldingType>),
    /// Visitor fields
    #[serde(rename = "defaultCollisionFilterInfo")]
    DefaultCollisionFilterInfo(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "cachedNumChildShapes")]
    CachedNumChildShapes(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "triangleRadius")]
    TriangleRadius(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "padding", skip_serializing)]
    Padding(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpExtendedMeshShapeVisitor<'de>, "@name",
    ("disableWelding" => DisableWelding(Primitive<bool>)),
    ("collectionType" => CollectionType(Primitive<CollectionType>)),
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("embeddedTrianglesSubpart" => EmbeddedTrianglesSubpart(SingleClass<HkpExtendedMeshShapeTrianglesSubpart<'de>>)),
    ("aabbHalfExtents" => AabbHalfExtents(Primitive<Vector4<f32>>)),
    ("aabbCenter" => AabbCenter(Primitive<Vector4<f32>>)),
    ("materialClass" => MaterialClass(Primitive<Cow<'de, str>>)),
    ("numBitsForSubpartIndex" => NumBitsForSubpartIndex(Primitive<i32>)),
    ("trianglesSubparts" => TrianglesSubparts(HkArrayClass<HkpExtendedMeshShapeTrianglesSubpart<'de>>)),
    ("shapesSubparts" => ShapesSubparts(HkArrayClass<HkpExtendedMeshShapeShapesSubpart<'de>>)),
    ("weldingInfo" => WeldingInfo(HkArrayNum<u16>)),
    ("weldingType" => WeldingType(Primitive<WeldingType>)),
    ("defaultCollisionFilterInfo" => DefaultCollisionFilterInfo(Primitive<u32>)),
    ("cachedNumChildShapes" => CachedNumChildShapes(Primitive<i32>)),
    ("triangleRadius" => TriangleRadius(Primitive<f32>)),
    ("padding" => Padding(Primitive<i32>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum IndexStridingType {
    #[serde(rename = "INDICES_INVALID")]
    #[default]
    IndicesInvalid = 0,
    #[serde(rename = "INDICES_INT8")]
    IndicesInt8 = 1,
    #[serde(rename = "INDICES_INT16")]
    IndicesInt16 = 2,
    #[serde(rename = "INDICES_INT32")]
    IndicesInt32 = 3,
    #[serde(rename = "INDICES_MAX_ID")]
    IndicesMaxId = 4,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum MaterialIndexStridingType {
    #[serde(rename = "MATERIAL_INDICES_INVALID")]
    #[default]
    MaterialIndicesInvalid = 0,
    #[serde(rename = "MATERIAL_INDICES_INT8")]
    MaterialIndicesInt8 = 1,
    #[serde(rename = "MATERIAL_INDICES_INT16")]
    MaterialIndicesInt16 = 2,
    #[serde(rename = "MATERIAL_INDICES_MAX_ID")]
    MaterialIndicesMaxId = 3,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum SubpartType {
    #[serde(rename = "SUBPART_TRIANGLES")]
    #[default]
    SubpartTriangles = 0,
    #[serde(rename = "SUBPART_SHAPE")]
    SubpartShape = 1,
}
