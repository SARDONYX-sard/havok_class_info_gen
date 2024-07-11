//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpCompressedMeshShapeBigTriangle`
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

/// `hkpCompressedMeshShapeBigTriangle`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0xcbfc95a4`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpCompressedMeshShapeBigTriangle {
    /// # C++ Class Fields Info
    /// -   name:`"a"`
    /// -   type: `hkUint16`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub a: u16,
    /// # C++ Class Fields Info
    /// -   name:`"b"`
    /// -   type: `hkUint16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    pub b: u16,
    /// # C++ Class Fields Info
    /// -   name:`"c"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub c: u16,
    /// # C++ Class Fields Info
    /// -   name:`"material"`
    /// -   type: `hkUint32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub material: u32,
    /// # C++ Class Fields Info
    /// -   name:`"weldingInfo"`
    /// -   type: `hkUint16`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub welding_info: u16,
    /// # C++ Class Fields Info
    /// -   name:`"transformIndex"`
    /// -   type: `hkUint16`
    /// - offset: 14
    /// -  flags: `FLAGS_NONE`
    pub transform_index: u16,
}

impl Serialize for HkpCompressedMeshShapeBigTriangle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpCompressedMeshShapeBigTriangleVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpCompressedMeshShapeBigTriangle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpCompressedMeshShapeBigTriangleVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpCompressedMeshShapeBigTriangleVisitor>> for HkpCompressedMeshShapeBigTriangle {
    fn from(_values: Vec<HkpCompressedMeshShapeBigTriangleVisitor>) -> Self {
            let mut a = None;
            let mut b = None;
            let mut c = None;
            let mut material = None;
            let mut welding_info = None;
            let mut transform_index = None;


        for _value in _values {
            match _value {
                HkpCompressedMeshShapeBigTriangleVisitor::A(m) => a = Some(m),
                HkpCompressedMeshShapeBigTriangleVisitor::B(m) => b = Some(m),
                HkpCompressedMeshShapeBigTriangleVisitor::C(m) => c = Some(m),
                HkpCompressedMeshShapeBigTriangleVisitor::Material(m) => material = Some(m),
                HkpCompressedMeshShapeBigTriangleVisitor::WeldingInfo(m) => welding_info = Some(m),
                HkpCompressedMeshShapeBigTriangleVisitor::TransformIndex(m) => transform_index = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            a: a.unwrap_or_default().into_inner(),
            b: b.unwrap_or_default().into_inner(),
            c: c.unwrap_or_default().into_inner(),
            material: material.unwrap_or_default().into_inner(),
            welding_info: welding_info.unwrap_or_default().into_inner(),
            transform_index: transform_index.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpCompressedMeshShapeBigTriangle> for Vec<HkpCompressedMeshShapeBigTriangleVisitor> {
    fn from(data: &HkpCompressedMeshShapeBigTriangle) -> Self {
        vec![
            HkpCompressedMeshShapeBigTriangleVisitor::A(data.a.into()),
            HkpCompressedMeshShapeBigTriangleVisitor::B(data.b.into()),
            HkpCompressedMeshShapeBigTriangleVisitor::C(data.c.into()),
            HkpCompressedMeshShapeBigTriangleVisitor::Material(data.material.into()),
            HkpCompressedMeshShapeBigTriangleVisitor::WeldingInfo(data.welding_info.into()),
            HkpCompressedMeshShapeBigTriangleVisitor::TransformIndex(data.transform_index.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpCompressedMeshShapeBigTriangle {
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
enum HkpCompressedMeshShapeBigTriangleVisitor {
    /// Visitor fields
    #[serde(rename = "a")]
    A(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "b")]
    B(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "c")]
    C(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "material")]
    Material(Primitive<u32>),
    /// Visitor fields
    #[serde(rename = "weldingInfo")]
    WeldingInfo(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "transformIndex")]
    TransformIndex(Primitive<u16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpCompressedMeshShapeBigTriangleVisitor, "@name",
    ("a" => A(Primitive<u16>)),
    ("b" => B(Primitive<u16>)),
    ("c" => C(Primitive<u16>)),
    ("material" => Material(Primitive<u32>)),
    ("weldingInfo" => WeldingInfo(Primitive<u16>)),
    ("transformIndex" => TransformIndex(Primitive<u16>)),
}
