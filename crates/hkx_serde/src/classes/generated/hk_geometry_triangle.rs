//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkGeometryTriangle`
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

/// `hkGeometryTriangle`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x9687513b`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkGeometryTriangle {
    /// # C++ Class Fields Info
    /// -   name:`"a"`
    /// -   type: `hkInt32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub a: i32,
    /// # C++ Class Fields Info
    /// -   name:`"b"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub b: i32,
    /// # C++ Class Fields Info
    /// -   name:`"c"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub c: i32,
    /// # C++ Class Fields Info
    /// -   name:`"material"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub material: i32,
}

impl Serialize for HkGeometryTriangle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkGeometryTriangleVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkGeometryTriangle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkGeometryTriangleVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkGeometryTriangleVisitor>> for HkGeometryTriangle {
    fn from(_values: Vec<HkGeometryTriangleVisitor>) -> Self {
            let mut a = None;
            let mut b = None;
            let mut c = None;
            let mut material = None;


        for _value in _values {
            match _value {
                HkGeometryTriangleVisitor::A(m) => a = Some(m),
                HkGeometryTriangleVisitor::B(m) => b = Some(m),
                HkGeometryTriangleVisitor::C(m) => c = Some(m),
                HkGeometryTriangleVisitor::Material(m) => material = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            a: a.unwrap_or_default().into_inner(),
            b: b.unwrap_or_default().into_inner(),
            c: c.unwrap_or_default().into_inner(),
            material: material.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkGeometryTriangle> for Vec<HkGeometryTriangleVisitor> {
    fn from(data: &HkGeometryTriangle) -> Self {
        vec![
            HkGeometryTriangleVisitor::A(data.a.into()),
            HkGeometryTriangleVisitor::B(data.b.into()),
            HkGeometryTriangleVisitor::C(data.c.into()),
            HkGeometryTriangleVisitor::Material(data.material.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkGeometryTriangle {
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
enum HkGeometryTriangleVisitor {
    /// Visitor fields
    #[serde(rename = "a")]
    A(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "b")]
    B(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "c")]
    C(Primitive<i32>),
    /// Visitor fields
    #[serde(rename = "material")]
    Material(Primitive<i32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkGeometryTriangleVisitor, "@name",
    ("a" => A(Primitive<i32>)),
    ("b" => B(Primitive<i32>)),
    ("c" => C(Primitive<i32>)),
    ("material" => Material(Primitive<i32>)),
}
