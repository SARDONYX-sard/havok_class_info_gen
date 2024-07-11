//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpStorageExtendedMeshShape`
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

/// `hkpStorageExtendedMeshShape`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 272
/// -    vtable: true
/// -    parent: `hkpExtendedMeshShape`/`0x177114a2`
/// - signature: `0xb469efbc`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpStorageExtendedMeshShape<'a> {
    /// # C++ Parent class(`hkpExtendedMeshShape` => parent: `hkpShapeCollection`) field Info
    /// -   name:`"embeddedTrianglesSubpart"`
    /// -   type: `struct hkpExtendedMeshShapeTrianglesSubpart`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub embedded_triangles_subpart: SingleClass<HkpExtendedMeshShapeTrianglesSubpart<'a>>,
    /// # C++ Parent class(`hkpExtendedMeshShape` => parent: `hkpShapeCollection`) field Info
    /// -   name:`"aabbHalfExtents"`
    /// -   type: `hkVector4`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    pub aabb_half_extents: Vector4<f32>,
    /// # C++ Parent class(`hkpExtendedMeshShape` => parent: `hkpShapeCollection`) field Info
    /// -   name:`"aabbCenter"`
    /// -   type: `hkVector4`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    pub aabb_center: Vector4<f32>,
    /// # C++ Parent class(`hkpExtendedMeshShape` => parent: `hkpShapeCollection`) field Info
    /// -   name:`"materialClass"`
    /// -   type: `void*`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub material_class: Cow<'a, str>,
    /// # C++ Parent class(`hkpExtendedMeshShape` => parent: `hkpShapeCollection`) field Info
    /// -   name:`"numBitsForSubpartIndex"`
    /// -   type: `hkInt32`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    pub num_bits_for_subpart_index: i32,
    /// # C++ Parent class(`hkpExtendedMeshShape` => parent: `hkpShapeCollection`) field Info
    /// -   name:`"trianglesSubparts"`
    /// -   type: `hkArray<struct hkpExtendedMeshShapeTrianglesSubpart>`
    /// - offset: 184
    /// -  flags: `FLAGS_NONE`
    pub triangles_subparts: HkArrayClass<HkpExtendedMeshShapeTrianglesSubpart<'a>>,
    /// # C++ Parent class(`hkpExtendedMeshShape` => parent: `hkpShapeCollection`) field Info
    /// -   name:`"shapesSubparts"`
    /// -   type: `hkArray<struct hkpExtendedMeshShapeShapesSubpart>`
    /// - offset: 196
    /// -  flags: `FLAGS_NONE`
    pub shapes_subparts: HkArrayClass<HkpExtendedMeshShapeShapesSubpart<'a>>,
    /// # C++ Parent class(`hkpExtendedMeshShape` => parent: `hkpShapeCollection`) field Info
    /// -   name:`"weldingInfo"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    pub welding_info: HkArrayNum<u16>,
    /// # C++ Parent class(`hkpExtendedMeshShape` => parent: `hkpShapeCollection`) field Info
    /// -   name:`"weldingType"`
    /// -   type: `enum WeldingType`
    /// - offset: 220
    /// -  flags: `FLAGS_NONE`
    pub welding_type: WeldingType,
    /// # C++ Parent class(`hkpExtendedMeshShape` => parent: `hkpShapeCollection`) field Info
    /// -   name:`"defaultCollisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 224
    /// -  flags: `FLAGS_NONE`
    pub default_collision_filter_info: u32,
    /// # C++ Parent class(`hkpExtendedMeshShape` => parent: `hkpShapeCollection`) field Info
    /// -   name:`"cachedNumChildShapes"`
    /// -   type: `hkInt32`
    /// - offset: 228
    /// -  flags: `FLAGS_NONE`
    pub cached_num_child_shapes: i32,
    /// # C++ Parent class(`hkpExtendedMeshShape` => parent: `hkpShapeCollection`) field Info
    /// -   name:`"triangleRadius"`
    /// -   type: `hkReal`
    /// - offset: 232
    /// -  flags: `FLAGS_NONE`
    pub triangle_radius: f32,
    /// # C++ Parent class(`hkpExtendedMeshShape` => parent: `hkpShapeCollection`) field Info
    /// -   name:`"padding"`
    /// -   type: `hkInt32`
    /// - offset: 236
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub padding: i32,

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
    /// -   name:`"meshstorage"`
    /// -   type: `hkArray<hkpStorageExtendedMeshShapeMeshSubpartStorage*>`
    /// - offset: 240
    /// -  flags: `FLAGS_NONE`
    pub meshstorage: HkArrayRef<Cow<'a, str>>,
    /// # C++ Class Fields Info
    /// -   name:`"shapestorage"`
    /// -   type: `hkArray<hkpStorageExtendedMeshShapeShapeSubpartStorage*>`
    /// - offset: 252
    /// -  flags: `FLAGS_NONE`
    pub shapestorage: HkArrayRef<Cow<'a, str>>,
}

impl Serialize for HkpStorageExtendedMeshShape<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpStorageExtendedMeshShapeVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpStorageExtendedMeshShape<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpStorageExtendedMeshShapeVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpStorageExtendedMeshShapeVisitor<'a>>> for HkpStorageExtendedMeshShape<'a> {
    fn from(_values: Vec<HkpStorageExtendedMeshShapeVisitor<'a>>) -> Self {
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
            let mut disable_welding = None;
            let mut collection_type = None;
            let mut user_data = None;
            let mut _type = None;
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut meshstorage = None;
            let mut shapestorage = None;


        for _value in _values {
            match _value {
                HkpStorageExtendedMeshShapeVisitor::EmbeddedTrianglesSubpart(m) => embedded_triangles_subpart = Some(m),
                HkpStorageExtendedMeshShapeVisitor::AabbHalfExtents(m) => aabb_half_extents = Some(m),
                HkpStorageExtendedMeshShapeVisitor::AabbCenter(m) => aabb_center = Some(m),
                HkpStorageExtendedMeshShapeVisitor::MaterialClass(m) => material_class = Some(m),
                HkpStorageExtendedMeshShapeVisitor::NumBitsForSubpartIndex(m) => num_bits_for_subpart_index = Some(m),
                HkpStorageExtendedMeshShapeVisitor::TrianglesSubparts(m) => triangles_subparts = Some(m),
                HkpStorageExtendedMeshShapeVisitor::ShapesSubparts(m) => shapes_subparts = Some(m),
                HkpStorageExtendedMeshShapeVisitor::WeldingInfo(m) => welding_info = Some(m),
                HkpStorageExtendedMeshShapeVisitor::WeldingType(m) => welding_type = Some(m),
                HkpStorageExtendedMeshShapeVisitor::DefaultCollisionFilterInfo(m) => default_collision_filter_info = Some(m),
                HkpStorageExtendedMeshShapeVisitor::CachedNumChildShapes(m) => cached_num_child_shapes = Some(m),
                HkpStorageExtendedMeshShapeVisitor::TriangleRadius(m) => triangle_radius = Some(m),
                HkpStorageExtendedMeshShapeVisitor::Padding(m) => padding = Some(m),
                HkpStorageExtendedMeshShapeVisitor::DisableWelding(m) => disable_welding = Some(m),
                HkpStorageExtendedMeshShapeVisitor::CollectionType(m) => collection_type = Some(m),
                HkpStorageExtendedMeshShapeVisitor::UserData(m) => user_data = Some(m),
                HkpStorageExtendedMeshShapeVisitor::Type(m) => _type = Some(m),
                HkpStorageExtendedMeshShapeVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpStorageExtendedMeshShapeVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpStorageExtendedMeshShapeVisitor::Meshstorage(m) => meshstorage = Some(m),
                HkpStorageExtendedMeshShapeVisitor::Shapestorage(m) => shapestorage = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
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
            disable_welding: disable_welding.unwrap_or_default().into_inner(),
            collection_type: collection_type.unwrap_or_default().into_inner(),
            user_data: user_data.unwrap_or_default().into_inner(),
            _type: _type.unwrap_or_default().into_inner(),
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            meshstorage: meshstorage.unwrap_or_default(),
            shapestorage: shapestorage.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpStorageExtendedMeshShape<'a>> for Vec<HkpStorageExtendedMeshShapeVisitor<'a>> {
    fn from(data: &HkpStorageExtendedMeshShape<'a>) -> Self {
        vec![
            HkpStorageExtendedMeshShapeVisitor::EmbeddedTrianglesSubpart(data.embedded_triangles_subpart.clone()),
            HkpStorageExtendedMeshShapeVisitor::AabbHalfExtents(data.aabb_half_extents.into()),
            HkpStorageExtendedMeshShapeVisitor::AabbCenter(data.aabb_center.into()),
            HkpStorageExtendedMeshShapeVisitor::MaterialClass(data.material_class.clone().into()),
            HkpStorageExtendedMeshShapeVisitor::NumBitsForSubpartIndex(data.num_bits_for_subpart_index.into()),
            HkpStorageExtendedMeshShapeVisitor::TrianglesSubparts(data.triangles_subparts.clone()),
            HkpStorageExtendedMeshShapeVisitor::ShapesSubparts(data.shapes_subparts.clone()),
            HkpStorageExtendedMeshShapeVisitor::WeldingInfo(data.welding_info.clone()),
            HkpStorageExtendedMeshShapeVisitor::WeldingType(data.welding_type.clone().into()),
            HkpStorageExtendedMeshShapeVisitor::DefaultCollisionFilterInfo(data.default_collision_filter_info.into()),
            HkpStorageExtendedMeshShapeVisitor::CachedNumChildShapes(data.cached_num_child_shapes.into()),
            HkpStorageExtendedMeshShapeVisitor::TriangleRadius(data.triangle_radius.into()),
            HkpStorageExtendedMeshShapeVisitor::Padding(data.padding.into()),
            HkpStorageExtendedMeshShapeVisitor::DisableWelding(data.disable_welding.into()),
            HkpStorageExtendedMeshShapeVisitor::CollectionType(data.collection_type.clone().into()),
            HkpStorageExtendedMeshShapeVisitor::UserData(data.user_data.into()),
            HkpStorageExtendedMeshShapeVisitor::Type(data._type.into()),
            HkpStorageExtendedMeshShapeVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpStorageExtendedMeshShapeVisitor::ReferenceCount(data.reference_count.into()),
            HkpStorageExtendedMeshShapeVisitor::Meshstorage(data.meshstorage.clone()),
            HkpStorageExtendedMeshShapeVisitor::Shapestorage(data.shapestorage.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpStorageExtendedMeshShape<'de> {
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
enum HkpStorageExtendedMeshShapeVisitor<'a> {
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
    #[serde(rename = "meshstorage")]
    Meshstorage(HkArrayRef<Cow<'a, str>>),
    /// Visitor fields
    #[serde(rename = "shapestorage")]
    Shapestorage(HkArrayRef<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpStorageExtendedMeshShapeVisitor<'de>, "@name",
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
    ("disableWelding" => DisableWelding(Primitive<bool>)),
    ("collectionType" => CollectionType(Primitive<CollectionType>)),
    ("userData" => UserData(Primitive<usize>)),
    ("type" => Type(Primitive<()>)),
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("meshstorage" => Meshstorage(HkArrayRef<Cow<'de, str>>)),
    ("shapestorage" => Shapestorage(HkArrayRef<Cow<'de, str>>)),
}
