//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpStorageMeshShapeSubpartStorage`
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

/// `hkpStorageMeshShapeSubpartStorage`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 80
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0xbf27438`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpStorageMeshShapeSubpartStorage {
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
    /// -   type: `hkArray<hkReal>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub vertices: HkArrayNum<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"indices16"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub indices_16: HkArrayNum<u16>,
    /// # C++ Class Fields Info
    /// -   name:`"indices32"`
    /// -   type: `hkArray<hkUint32>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub indices_32: HkArrayNum<u32>,
    /// # C++ Class Fields Info
    /// -   name:`"materialIndices"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub material_indices: HkArrayNum<u8>,
    /// # C++ Class Fields Info
    /// -   name:`"materials"`
    /// -   type: `hkArray<hkUint32>`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub materials: HkArrayNum<u32>,
    /// # C++ Class Fields Info
    /// -   name:`"materialIndices16"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub material_indices_16: HkArrayNum<u16>,
}

impl Serialize for HkpStorageMeshShapeSubpartStorage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpStorageMeshShapeSubpartStorageVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpStorageMeshShapeSubpartStorage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpStorageMeshShapeSubpartStorageVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpStorageMeshShapeSubpartStorageVisitor>> for HkpStorageMeshShapeSubpartStorage {
    fn from(_values: Vec<HkpStorageMeshShapeSubpartStorageVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut vertices = None;
            let mut indices_16 = None;
            let mut indices_32 = None;
            let mut material_indices = None;
            let mut materials = None;
            let mut material_indices_16 = None;


        for _value in _values {
            match _value {
                HkpStorageMeshShapeSubpartStorageVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpStorageMeshShapeSubpartStorageVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpStorageMeshShapeSubpartStorageVisitor::Vertices(m) => vertices = Some(m),
                HkpStorageMeshShapeSubpartStorageVisitor::Indices16(m) => indices_16 = Some(m),
                HkpStorageMeshShapeSubpartStorageVisitor::Indices32(m) => indices_32 = Some(m),
                HkpStorageMeshShapeSubpartStorageVisitor::MaterialIndices(m) => material_indices = Some(m),
                HkpStorageMeshShapeSubpartStorageVisitor::Materials(m) => materials = Some(m),
                HkpStorageMeshShapeSubpartStorageVisitor::MaterialIndices16(m) => material_indices_16 = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            vertices: vertices.unwrap_or_default(),
            indices_16: indices_16.unwrap_or_default(),
            indices_32: indices_32.unwrap_or_default(),
            material_indices: material_indices.unwrap_or_default(),
            materials: materials.unwrap_or_default(),
            material_indices_16: material_indices_16.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpStorageMeshShapeSubpartStorage> for Vec<HkpStorageMeshShapeSubpartStorageVisitor> {
    fn from(data: &HkpStorageMeshShapeSubpartStorage) -> Self {
        vec![
            HkpStorageMeshShapeSubpartStorageVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpStorageMeshShapeSubpartStorageVisitor::ReferenceCount(data.reference_count.into()),
            HkpStorageMeshShapeSubpartStorageVisitor::Vertices(data.vertices.clone()),
            HkpStorageMeshShapeSubpartStorageVisitor::Indices16(data.indices_16.clone()),
            HkpStorageMeshShapeSubpartStorageVisitor::Indices32(data.indices_32.clone()),
            HkpStorageMeshShapeSubpartStorageVisitor::MaterialIndices(data.material_indices.clone()),
            HkpStorageMeshShapeSubpartStorageVisitor::Materials(data.materials.clone()),
            HkpStorageMeshShapeSubpartStorageVisitor::MaterialIndices16(data.material_indices_16.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpStorageMeshShapeSubpartStorage {
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
enum HkpStorageMeshShapeSubpartStorageVisitor {
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
    Vertices(HkArrayNum<f32>),
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
    Materials(HkArrayNum<u32>),
    /// Visitor fields
    #[serde(rename = "materialIndices16")]
    MaterialIndices16(HkArrayNum<u16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpStorageMeshShapeSubpartStorageVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("vertices" => Vertices(HkArrayNum<f32>)),
    ("indices16" => Indices16(HkArrayNum<u16>)),
    ("indices32" => Indices32(HkArrayNum<u32>)),
    ("materialIndices" => MaterialIndices(HkArrayNum<u8>)),
    ("materials" => Materials(HkArrayNum<u32>)),
    ("materialIndices16" => MaterialIndices16(HkArrayNum<u16>)),
}
