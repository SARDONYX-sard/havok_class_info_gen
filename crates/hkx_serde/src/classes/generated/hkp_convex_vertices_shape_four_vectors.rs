//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpConvexVerticesShapeFourVectors`
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

/// `hkpConvexVerticesShapeFourVectors`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: false
/// - signature: `0x3d80c5bf`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkpConvexVerticesShapeFourVectors {
    /// # C++ Class Fields Info
    /// -   name:`"x"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub x: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"y"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub y: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"z"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub z: Vector4<f32>,
}

impl Serialize for HkpConvexVerticesShapeFourVectors {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkpConvexVerticesShapeFourVectorsVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkpConvexVerticesShapeFourVectors {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkpConvexVerticesShapeFourVectorsVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkpConvexVerticesShapeFourVectorsVisitor>> for HkpConvexVerticesShapeFourVectors {
    fn from(_values: Vec<HkpConvexVerticesShapeFourVectorsVisitor>) -> Self {
            let mut x = None;
            let mut y = None;
            let mut z = None;


        for _value in _values {
            match _value {
                HkpConvexVerticesShapeFourVectorsVisitor::X(m) => x = Some(m),
                HkpConvexVerticesShapeFourVectorsVisitor::Y(m) => y = Some(m),
                HkpConvexVerticesShapeFourVectorsVisitor::Z(m) => z = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            x: x.unwrap_or_default().into_inner(),
            y: y.unwrap_or_default().into_inner(),
            z: z.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkpConvexVerticesShapeFourVectors> for Vec<HkpConvexVerticesShapeFourVectorsVisitor> {
    fn from(data: &HkpConvexVerticesShapeFourVectors) -> Self {
        vec![
            HkpConvexVerticesShapeFourVectorsVisitor::X(data.x.into()),
            HkpConvexVerticesShapeFourVectorsVisitor::Y(data.y.into()),
            HkpConvexVerticesShapeFourVectorsVisitor::Z(data.z.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpConvexVerticesShapeFourVectors {
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
enum HkpConvexVerticesShapeFourVectorsVisitor {
    /// Visitor fields
    #[serde(rename = "x")]
    X(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "y")]
    Y(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "z")]
    Z(Primitive<Vector4<f32>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpConvexVerticesShapeFourVectorsVisitor, "@name",
    ("x" => X(Primitive<Vector4<f32>>)),
    ("y" => Y(Primitive<Vector4<f32>>)),
    ("z" => Z(Primitive<Vector4<f32>>)),
}
