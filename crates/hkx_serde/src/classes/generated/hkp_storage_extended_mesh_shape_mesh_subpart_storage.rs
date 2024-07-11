//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpStorageExtendedMeshShapeMeshSubpartStorage`
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

/// `hkpStorageExtendedMeshShapeMeshSubpartStorage`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 104
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x5aad4de6`
/// -   version: 3
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpStorageExtendedMeshShapeMeshSubpartStorage<'a> {
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
    /// -   name:`"vertices"`
    /// -   type: `hkArray<hkVector4>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub vertices: HkArrayVector<Vector4<f32>>,
    /// # C++ Class Fields Info
    /// -   name:`"indices8"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub indices_8: HkArrayNum<u8>,
    /// # C++ Class Fields Info
    /// -   name:`"indices16"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub indices_16: HkArrayNum<u16>,
    /// # C++ Class Fields Info
    /// -   name:`"indices32"`
    /// -   type: `hkArray<hkUint32>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub indices_32: HkArrayNum<u32>,
    /// # C++ Class Fields Info
    /// -   name:`"materialIndices"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub material_indices: HkArrayNum<u8>,
    /// # C++ Class Fields Info
    /// -   name:`"materials"`
    /// -   type: `hkArray<struct hkpStorageExtendedMeshShapeMaterial>`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub materials: HkArrayClass<HkpStorageExtendedMeshShapeMaterial>,
    /// # C++ Class Fields Info
    /// -   name:`"namedMaterials"`
    /// -   type: `hkArray<struct hkpNamedMeshMaterial>`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub named_materials: HkArrayClass<HkpNamedMeshMaterial<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"materialIndices16"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    pub material_indices_16: HkArrayNum<u16>,
}

impl Serialize for HkpStorageExtendedMeshShapeMeshSubpartStorage<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpStorageExtendedMeshShapeMeshSubpartStorage<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor<'a>>> for HkpStorageExtendedMeshShapeMeshSubpartStorage<'a> {
    fn from(_values: Vec<HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor<'a>>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut vertices = None;
            let mut indices_8 = None;
            let mut indices_16 = None;
            let mut indices_32 = None;
            let mut material_indices = None;
            let mut materials = None;
            let mut named_materials = None;
            let mut material_indices_16 = None;


        for _value in _values {
            match _value {
                HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor::Vertices(m) => vertices = Some(m),
                HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor::Indices8(m) => indices_8 = Some(m),
                HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor::Indices16(m) => indices_16 = Some(m),
                HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor::Indices32(m) => indices_32 = Some(m),
                HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor::MaterialIndices(m) => material_indices = Some(m),
                HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor::Materials(m) => materials = Some(m),
                HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor::NamedMaterials(m) => named_materials = Some(m),
                HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor::MaterialIndices16(m) => material_indices_16 = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            vertices: vertices.unwrap_or_default(),
            indices_8: indices_8.unwrap_or_default(),
            indices_16: indices_16.unwrap_or_default(),
            indices_32: indices_32.unwrap_or_default(),
            material_indices: material_indices.unwrap_or_default(),
            materials: materials.unwrap_or_default(),
            named_materials: named_materials.unwrap_or_default(),
            material_indices_16: material_indices_16.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkpStorageExtendedMeshShapeMeshSubpartStorage<'a>> for Vec<HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor<'a>> {
    fn from(data: &HkpStorageExtendedMeshShapeMeshSubpartStorage<'a>) -> Self {
        vec![
            HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor::ReferenceCount(data.reference_count.into()),
            HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor::Vertices(data.vertices.clone()),
            HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor::Indices8(data.indices_8.clone()),
            HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor::Indices16(data.indices_16.clone()),
            HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor::Indices32(data.indices_32.clone()),
            HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor::MaterialIndices(data.material_indices.clone()),
            HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor::Materials(data.materials.clone()),
            HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor::NamedMaterials(data.named_materials.clone()),
            HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor::MaterialIndices16(data.material_indices_16.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpStorageExtendedMeshShapeMeshSubpartStorage<'de> {
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
enum HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "vertices")]
    Vertices(HkArrayVector<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "indices8")]
    Indices8(HkArrayNum<u8>),
    /// Visitor fields
    #[serde(rename = "indices16")]
    Indices16(HkArrayNum<u16>),
    /// Visitor fields
    #[serde(rename = "indices32")]
    Indices32(HkArrayNum<u32>),
    /// Visitor fields
    #[serde(rename = "materialIndices")]
    MaterialIndices(HkArrayNum<u8>),
    /// Visitor fields
    #[serde(rename = "materials")]
    Materials(HkArrayClass<HkpStorageExtendedMeshShapeMaterial>),
    /// Visitor fields
    #[serde(rename = "namedMaterials")]
    NamedMaterials(HkArrayClass<HkpNamedMeshMaterial<'a>>),
    /// Visitor fields
    #[serde(rename = "materialIndices16")]
    MaterialIndices16(HkArrayNum<u16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpStorageExtendedMeshShapeMeshSubpartStorageVisitor<'de>, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("vertices" => Vertices(HkArrayVector<Vector4<f32>>)),
    ("indices8" => Indices8(HkArrayNum<u8>)),
    ("indices16" => Indices16(HkArrayNum<u16>)),
    ("indices32" => Indices32(HkArrayNum<u32>)),
    ("materialIndices" => MaterialIndices(HkArrayNum<u8>)),
    ("materials" => Materials(HkArrayClass<HkpStorageExtendedMeshShapeMaterial>)),
    ("namedMaterials" => NamedMaterials(HkArrayClass<HkpNamedMeshMaterial<'de>>)),
    ("materialIndices16" => MaterialIndices16(HkArrayNum<u16>)),
}
