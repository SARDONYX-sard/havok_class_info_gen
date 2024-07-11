//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpConvexVerticesConnectivity`
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

/// `hkpConvexVerticesConnectivity`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x63d38e9c`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpConvexVerticesConnectivity {
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
    /// -   name:`"vertexIndices"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub vertex_indices: HkArrayNum<u16>,
    /// # C++ Class Fields Info
    /// -   name:`"numVerticesPerFace"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub num_vertices_per_face: HkArrayNum<u8>,
}

impl Serialize for HkpConvexVerticesConnectivity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpConvexVerticesConnectivityVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpConvexVerticesConnectivity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpConvexVerticesConnectivityVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpConvexVerticesConnectivityVisitor>> for HkpConvexVerticesConnectivity {
    fn from(_values: Vec<HkpConvexVerticesConnectivityVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut vertex_indices = None;
            let mut num_vertices_per_face = None;


        for _value in _values {
            match _value {
                HkpConvexVerticesConnectivityVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkpConvexVerticesConnectivityVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkpConvexVerticesConnectivityVisitor::VertexIndices(m) => vertex_indices = Some(m),
                HkpConvexVerticesConnectivityVisitor::NumVerticesPerFace(m) => num_vertices_per_face = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            vertex_indices: vertex_indices.unwrap_or_default(),
            num_vertices_per_face: num_vertices_per_face.unwrap_or_default(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpConvexVerticesConnectivity> for Vec<HkpConvexVerticesConnectivityVisitor> {
    fn from(data: &HkpConvexVerticesConnectivity) -> Self {
        vec![
            HkpConvexVerticesConnectivityVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkpConvexVerticesConnectivityVisitor::ReferenceCount(data.reference_count.into()),
            HkpConvexVerticesConnectivityVisitor::VertexIndices(data.vertex_indices.clone()),
            HkpConvexVerticesConnectivityVisitor::NumVerticesPerFace(data.num_vertices_per_face.clone()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpConvexVerticesConnectivity {
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
enum HkpConvexVerticesConnectivityVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "vertexIndices")]
    VertexIndices(HkArrayNum<u16>),
    /// Visitor fields
    #[serde(rename = "numVerticesPerFace")]
    NumVerticesPerFace(HkArrayNum<u8>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConvexVerticesConnectivityVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("vertexIndices" => VertexIndices(HkArrayNum<u16>)),
    ("numVerticesPerFace" => NumVerticesPerFace(HkArrayNum<u8>)),
}
