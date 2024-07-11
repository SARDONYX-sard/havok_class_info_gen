//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkxVertexBufferVertexData`
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

/// `hkxVertexBufferVertexData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 84
/// -    vtable: false
/// - signature: `0xd72b6fd0`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkxVertexBufferVertexData {
    /// # C++ Class Fields Info
    /// -   name:`"vectorData"`
    /// -   type: `hkArray<hkVector4>`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub vector_data: HkArrayVector<Vector4<f32>>,
    /// # C++ Class Fields Info
    /// -   name:`"floatData"`
    /// -   type: `hkArray<hkReal>`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub float_data: HkArrayNum<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"uint32Data"`
    /// -   type: `hkArray<hkUint32>`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub uint_32_data: HkArrayNum<u32>,
    /// # C++ Class Fields Info
    /// -   name:`"uint16Data"`
    /// -   type: `hkArray<hkUint16>`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub uint_16_data: HkArrayNum<u16>,
    /// # C++ Class Fields Info
    /// -   name:`"uint8Data"`
    /// -   type: `hkArray<hkUint8>`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub uint_8_data: HkArrayNum<u8>,
    /// # C++ Class Fields Info
    /// -   name:`"numVerts"`
    /// -   type: `hkUint32`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub num_verts: u32,
    /// # C++ Class Fields Info
    /// -   name:`"vectorStride"`
    /// -   type: `hkUint32`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub vector_stride: u32,
    /// # C++ Class Fields Info
    /// -   name:`"floatStride"`
    /// -   type: `hkUint32`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub float_stride: u32,
    /// # C++ Class Fields Info
    /// -   name:`"uint32Stride"`
    /// -   type: `hkUint32`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    pub uint_32_stride: u32,
    /// # C++ Class Fields Info
    /// -   name:`"uint16Stride"`
    /// -   type: `hkUint32`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    pub uint_16_stride: u32,
    /// # C++ Class Fields Info
    /// -   name:`"uint8Stride"`
    /// -   type: `hkUint32`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub uint_8_stride: u32,
}

impl Serialize for HkxVertexBufferVertexData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkxVertexBufferVertexDataVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkxVertexBufferVertexData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkxVertexBufferVertexDataVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkxVertexBufferVertexDataVisitor>> for HkxVertexBufferVertexData {
    fn from(_values: Vec<HkxVertexBufferVertexDataVisitor>) -> Self {
            let mut vector_data = None;
            let mut float_data = None;
            let mut uint_32_data = None;
            let mut uint_16_data = None;
            let mut uint_8_data = None;
            let mut num_verts = None;
            let mut vector_stride = None;
            let mut float_stride = None;
            let mut uint_32_stride = None;
            let mut uint_16_stride = None;
            let mut uint_8_stride = None;


        for _value in _values {
            match _value {
                HkxVertexBufferVertexDataVisitor::VectorData(m) => vector_data = Some(m),
                HkxVertexBufferVertexDataVisitor::FloatData(m) => float_data = Some(m),
                HkxVertexBufferVertexDataVisitor::Uint32Data(m) => uint_32_data = Some(m),
                HkxVertexBufferVertexDataVisitor::Uint16Data(m) => uint_16_data = Some(m),
                HkxVertexBufferVertexDataVisitor::Uint8Data(m) => uint_8_data = Some(m),
                HkxVertexBufferVertexDataVisitor::NumVerts(m) => num_verts = Some(m),
                HkxVertexBufferVertexDataVisitor::VectorStride(m) => vector_stride = Some(m),
                HkxVertexBufferVertexDataVisitor::FloatStride(m) => float_stride = Some(m),
                HkxVertexBufferVertexDataVisitor::Uint32Stride(m) => uint_32_stride = Some(m),
                HkxVertexBufferVertexDataVisitor::Uint16Stride(m) => uint_16_stride = Some(m),
                HkxVertexBufferVertexDataVisitor::Uint8Stride(m) => uint_8_stride = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            vector_data: vector_data.unwrap_or_default(),
            float_data: float_data.unwrap_or_default(),
            uint_32_data: uint_32_data.unwrap_or_default(),
            uint_16_data: uint_16_data.unwrap_or_default(),
            uint_8_data: uint_8_data.unwrap_or_default(),
            num_verts: num_verts.unwrap_or_default().into_inner(),
            vector_stride: vector_stride.unwrap_or_default().into_inner(),
            float_stride: float_stride.unwrap_or_default().into_inner(),
            uint_32_stride: uint_32_stride.unwrap_or_default().into_inner(),
            uint_16_stride: uint_16_stride.unwrap_or_default().into_inner(),
            uint_8_stride: uint_8_stride.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkxVertexBufferVertexData> for Vec<HkxVertexBufferVertexDataVisitor> {
    fn from(data: &HkxVertexBufferVertexData) -> Self {
        vec![
            HkxVertexBufferVertexDataVisitor::VectorData(data.vector_data.clone()),
            HkxVertexBufferVertexDataVisitor::FloatData(data.float_data.clone()),
            HkxVertexBufferVertexDataVisitor::Uint32Data(data.uint_32_data.clone()),
            HkxVertexBufferVertexDataVisitor::Uint16Data(data.uint_16_data.clone()),
            HkxVertexBufferVertexDataVisitor::Uint8Data(data.uint_8_data.clone()),
            HkxVertexBufferVertexDataVisitor::NumVerts(data.num_verts.into()),
            HkxVertexBufferVertexDataVisitor::VectorStride(data.vector_stride.into()),
            HkxVertexBufferVertexDataVisitor::FloatStride(data.float_stride.into()),
            HkxVertexBufferVertexDataVisitor::Uint32Stride(data.uint_32_stride.into()),
            HkxVertexBufferVertexDataVisitor::Uint16Stride(data.uint_16_stride.into()),
            HkxVertexBufferVertexDataVisitor::Uint8Stride(data.uint_8_stride.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkxVertexBufferVertexData {
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
enum HkxVertexBufferVertexDataVisitor {
    /// Visitor fields
    #[serde(rename = "vectorData")]
    VectorData(HkArrayVector<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "floatData")]
    FloatData(HkArrayNum<f32>),
    /// Visitor fields
    #[serde(rename = "uint32Data")]
    Uint32Data(HkArrayNum<u32>),
    /// Visitor fields
    #[serde(rename = "uint16Data")]
    Uint16Data(HkArrayNum<u16>),
    /// Visitor fields
    #[serde(rename = "uint8Data")]
    Uint8Data(HkArrayNum<u8>),
    /// Visitor fields
    #[serde(rename = "numVerts")]
    NumVerts(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "vectorStride")]
    VectorStride(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "floatStride")]
    FloatStride(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "uint32Stride")]
    Uint32Stride(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "uint16Stride")]
    Uint16Stride(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "uint8Stride")]
    Uint8Stride(Primitive<u32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkxVertexBufferVertexDataVisitor, "@name",
    ("vectorData" => VectorData(HkArrayVector<Vector4<f32>>)),
    ("floatData" => FloatData(HkArrayNum<f32>)),
    ("uint32Data" => Uint32Data(HkArrayNum<u32>)),
    ("uint16Data" => Uint16Data(HkArrayNum<u16>)),
    ("uint8Data" => Uint8Data(HkArrayNum<u8>)),
    ("numVerts" => NumVerts(Primitive<u32>)),
    ("vectorStride" => VectorStride(Primitive<u32>)),
    ("floatStride" => FloatStride(Primitive<u32>)),
    ("uint32Stride" => Uint32Stride(Primitive<u32>)),
    ("uint16Stride" => Uint16Stride(Primitive<u32>)),
    ("uint8Stride" => Uint8Stride(Primitive<u32>)),
}
